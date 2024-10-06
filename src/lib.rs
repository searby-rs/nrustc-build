#![allow(unused)]

use std::fmt;
use std::marker::PhantomData;
use std::path::Path;

pub mod const_global {
    pub const RUSTC: super::Rustc = super::Rustc::new();
}

pub mod static_global {
    pub static RUSTC: super::Rustc = super::Rustc::new();
}

#[derive(Debug, Clone, Copy)]
pub struct Rustc {
    _marker: PhantomData<usize>,
}

impl Rustc {
    
    #[inline]
    pub(crate) const fn new() -> Rustc {
        Rustc {
            _marker: PhantomData,
        }
    }

    pub fn println<F: FnOnce() -> D, D: fmt::Display>(&self, msg: F) {
        let out = msg();
        println!("{}", out)
    }

    pub fn println_cargo<F: FnOnce() -> D, D: fmt::Display>(&self, msg: F) {
        let out = msg();
        self.println(|| {
            format!("cargo::{}", out)
        })
    }

    pub fn println_clippy<F: FnOnce() -> D, D: fmt::Display>(&self, msg: F) {
        let out = msg();
        self.println_cargo(|| {
            format!("clippy-{}", out)
        })
    }

    pub fn println_rustc<F: FnOnce() -> D, D: fmt::Display>(&self, msg: F) {
        let out = msg();
        self.println_cargo(|| {
            format!("rustc-{}", out)
        })
    }

    pub fn rerun_if_changed<P: AsRef<Path>>(&self, path: P) {
        let path2: &Path = path.as_ref();
        let disp = path2.display();
        self.println_cargo(|| {
            format!("rerun-if-changed={}", disp)
        })
    }

    pub unsafe fn rustc_check_cfg_raw<F: FnOnce() -> D, D: fmt::Display>(&self, msg: F) {
        let out = msg();
        self.println_rustc(|| {
            format!("check-cfg={}", out)
        })
    }

    pub fn rustc_check_cfg<F: FnOnce() -> D, D: fmt::Display>(&self, msg: F) {
        unsafe {
            let out = msg();
            let out_str = out.to_string();
            self.rustc_check_cfg_raw(|| {
                if out_str.starts_with("cfg(") && out_str.ends_with(")") {
                    out_str
                } else {
                    format!("cfg({})", out_str)
                }
            })
        }
    }

    pub fn rustc_check_cfg_wrap_key(&self, cfg: &str) {
        self.rustc_check_cfg(|| format!("cfg({})", cfg))
    }

    pub fn rustc_check_cfg_wrap_key_value(&self, cfg: &str, value: &str) {
        self.rustc_check_cfg(|| format!("cfg({}, values(\"{}\"))", cfg, value))
    }

    pub fn rustc_check_cfg_wrap_key_values(&self, cfg: &str, values: &[&str]) {
        let mut iter = values.into_iter();

        while let Some(value) = iter.next() {
            self.rustc_check_cfg_wrap_key_value(cfg, value);
        }
    }

    pub fn rustc_cfg<F: FnOnce() -> D, D: fmt::Display>(&self, msg: F) {
        self.println_rustc(|| {
            let out = msg();
            format!("cfg={}", out)
        })
    }

    pub fn rustc_cfg_wrap_key(&self, cfg: &str) {
        self.rustc_cfg(|| {
            cfg
        })
    }

    pub fn rustc_cfg_wrap_key_if_env_set(&self, cfg: &str, env: &str) {
        let envu = env.to_uppercase();
        if std::env::var_os(&envu).is_some() {
            self.rustc_cfg_wrap_key(cfg);
        }
    }

    pub fn rustc_cfg_wrap_key_if_env_has_value(&self, cfg: &str, env: &str, env_value: &str) {
        let envu = env.to_uppercase();
        let vall = env_value.to_lowercase();

        if let Ok(envval) = std::env::var(envu) {
            if envval.to_lowercase() == vall {
                self.rustc_cfg_wrap_key(cfg);
            }
        }
    }

    pub fn rustc_cfg_wrap_key_value(&self, cfg: &str, value: &str) {
        self.rustc_cfg(|| format!("{}=\"{}\"", cfg, value))
    }
    
    pub fn rustc_cfg_wrap_key_value_if_env_set(&self, cfg: &str, value: &str, env: &str) {
        let envu = env.to_uppercase();
        if std::env::var_os(&envu).is_some() {
            self.rustc_cfg_wrap_key_value(cfg, value);
        }
    }

    pub fn rustc_cfg_wrap_key_value_if_env_has_value(&self, cfg: &str, value: &str, env: &str, env_value: &str) {
        let envu = env.to_uppercase();
        let vall = env_value.to_lowercase();

        if let Ok(envval) = std::env::var(envu) {
            if envval.to_lowercase() == vall {
                self.rustc_cfg_wrap_key_value(cfg, value)
            }
        }
    }

    pub fn rustc_cfg_wrap_key_values(&self, cfg: &str, values: &[&str]) {
        let mut iter = values.into_iter();
        
        while let Some(value) = iter.next() {
            self.rustc_cfg_wrap_key_value(cfg, value);
        }
    }

    pub fn rustc_cfg_wrap_key_values_if_env_set(&self, cfg: &str, values: &[&str], env: &str) {
        let envu = env.to_uppercase();

        if std::env::var_os(&envu).is_some() {
            self.rustc_cfg_wrap_key_values(cfg, values);
        }
    }

    pub fn rustc_cfg_wrap_key_values_if_env_has_value(&self, cfg: &str, values: &[&str], env: &str, env_value: &str) {
        let envu = env.to_uppercase();
        let vall = env_value.to_lowercase();

        if let Ok(envval) = std::env::var(envu) {
            if envval.to_lowercase() == vall {
                self.rustc_cfg_wrap_key_values(cfg, values);
            }
        }
    }
}
