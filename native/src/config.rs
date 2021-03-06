use crate::Compiler;
use fnv::FnvHashMap;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};
use swc::{
    atoms::JsWord,
    common::FileName,
    ecmascript::{
        ast::{Expr, ModuleItem, Stmt},
        parser::Syntax,
        transforms::{react, InlineGlobals},
    },
};

/// `.swcrc` file
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub jsc: JscConfig,
}

impl Config {}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct JscConfig {
    #[serde(rename = "parser", default)]
    pub syntax: Syntax,
    #[serde(default)]
    pub transform: TrnasformConfig,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TrnasformConfig {
    #[serde(default)]
    pub react: react::Options,
    #[serde(default)]
    pub optimizer: Option<OptimizerConfig>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OptimizerConfig {
    #[serde(default)]
    pub globals: Option<GlobalPassOption>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct GlobalPassOption {
    #[serde(default)]
    pub vars: FnvHashMap<String, String>,
}

impl GlobalPassOption {
    pub fn build(self, c: &Compiler) -> InlineGlobals {
        fn mk_map(
            c: &Compiler,
            values: impl Iterator<Item = (String, String)>,
            is_env: bool,
        ) -> HashMap<JsWord, Expr> {
            let mut m = HashMap::new();

            for (k, v) in values {
                let v = if is_env {
                    format!("'{}'", v)
                } else {
                    (*v).into()
                };

                let mut module = c
                    .parse_js(
                        FileName::Custom(format!("GLOBAL_{}", k)),
                        Default::default(),
                        &v,
                    )
                    .unwrap_or_else(|err| panic!("failed to parse globals.{}: {:?}", k, err));
                let expr = match module.body.pop().unwrap() {
                    ModuleItem::Stmt(Stmt::Expr(box expr)) => expr,
                    _ => panic!("{} is not a valid expression", v),
                };

                m.insert((*k).into(), expr);
            }

            m
        }

        InlineGlobals {
            globals: mk_map(c, self.vars.into_iter(), false),
            envs: mk_map(c, env::vars(), true),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use ecmascript::parser::TsConfig;

    #[test]
    fn test() {
        println!(
            "{}",
            serde_json::to_string_pretty(&Config {
                jsc: JscConfig {
                    syntax: Syntax::Typescript(TsConfig {
                        ..Default::default()
                    }),
                    transform: TrnasformConfig {
                        react: react::Options {
                            ..Default::default()
                        },
                        optimizer: Some(OptimizerConfig {
                            globals: Some(GlobalPassOption {
                                vars: {
                                    let mut map = FnvHashMap::default();
                                    map.insert("__DEBUG__".into(), "true".into());
                                    map
                                }
                            }),
                        })
                    }
                },
            })
            .unwrap()
        );
    }

}
