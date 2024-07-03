// Copyright 2024 brian <gao.brian@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::{bail,Result};
use regorus;
use regorus::QueryResults;

pub struct Engine {
    pub engine: regorus::Engine,
    #[cfg(feature = "coverage")]
    pub enable_coverage: bool
}

impl Engine {
    pub fn new(enable_coverage: bool) -> Self {
        let mut engine = Self {
            engine: regorus::Engine::new(),
            #[cfg(feature = "coverage")]
            enable_coverage
        };

        #[cfg(feature = "coverage")]
        if enable_coverage {
            engine.engine.set_enable_coverage(true);
        }

        engine.engine.set_strict_builtin_errors(true);
        engine
    }

    //Add bundles from files
    pub fn add_bundles(&mut self, bundles: &[String]) -> Result<()> {
        // Load files from given bundles.
        for dir in bundles.iter() {
            let entries =
                std::fs::read_dir(dir).or_else(|e| bail!("failed to read bundle {dir}.\n{e}"))?;
            // Loop through each entry in the bundle folder.
            for entry in entries {
                let entry = entry.or_else(|e| bail!("failed to unwrap entry. {e}"))?;
                let path = entry.path();
                // Process only .rego files.
                match (path.is_file(), path.extension()) {
                    (true, Some(ext)) if ext == "rego" => {}
                    _ => continue,
                }
                println!("Loading bundle: {}", path.display());
                self.engine.add_policy_from_file(entry.path())?;
            }
        }

        Ok(())
    }

    //Add input from file
    pub fn add_input(&mut self,input: Option<String>,) -> Result<()> {
        if let Some(file) = input {
            let input = if file.ends_with(".json") {
                regorus::Value::from_json_file(&file)?
            } else if file.ends_with(".yaml") {
                regorus::Value::from_yaml_file(&file)?
            } else {
                bail!("Unsupported input file `{file}`. Must be json or yaml.")
            };
            self.engine.set_input(input);
        }
        Ok(())
    }

    pub fn add_input_json(&mut self, input_json: Option<String>) -> Result<()> {
        if let Some(input_json) = input_json {
            self.engine.set_input_json(&input_json)?;
        }
        Ok(())
    }

    //Add policy from file
    pub fn add_policy_from_file(&mut self, file: String) -> Result<()> {
        if file.ends_with(".rego") {
            // Read policy file.
            self.engine.add_policy_from_file(file)?;
        }
        Ok(())
    }

    //Add policy from string
    pub fn add_policy_from_string(&mut self,path: String, policy: String) -> Result<()> {
        self.engine.add_policy(path,policy)?;
        Ok(())
    }

    //Close engine
    pub fn close(&mut self) -> Result<()>{
        Ok(())
    }

    //Add data from file
    pub fn add_data(&mut self, data: Option<String>) -> Result<()> {
        if let Some(file) = data {
            // Read data file.
            let data = if file.ends_with(".json") {
                regorus::Value::from_json_file(file)?
            } else if file.ends_with(".yaml") {
                regorus::Value::from_yaml_file(&file)?
            } else {
                bail!("Unsupported data file `{file}`. Must be rego, json or yaml.")
            };

            // Merge given data.
            self.engine.add_data(data)?;
        }

        Ok(())
    }

    //Add data from string
    pub fn add_data_from_string(&mut self, data: String) -> Result<()> {
        self.engine.add_data_json(&data)?;
        Ok(())
    }

    //Clear data
    pub fn clear_data(&mut self) {
        self.engine.clear_data();
    }

    pub fn eval_query(&mut self, query: String,enable_tracing: bool) -> Result<QueryResults> {
        let result = self.engine.eval_query(query, enable_tracing);
        result
    }

    pub fn eval_rule(&mut self, path: String) -> Result<regorus::Value> {
        let result = self.engine.eval_rule(path);
        result
    }

    pub fn get_packages(&self) -> Result<Vec<String>> {
        self.engine.get_packages()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regorus::Engine as RegoEngine;
    use serde_json;

    #[test]
    fn test_eval_query() {
        let mut engine = Engine::new(true);

        let input = r#"
{
    "servers": [
        {"id": "app", "protocols": ["https", "ssh"], "ports": ["p1", "p2", "p3"]},
        {"id": "db", "protocols": ["mysql"], "ports": ["p3"]},
        {"id": "cache", "protocols": ["memcache"], "ports": ["p3"]},
        {"id": "ci", "protocols": ["http"], "ports": ["p1", "p2"]},
        {"id": "busybox", "protocols": ["telnet"], "ports": ["p1"]}
    ],
    "networks": [
        {"id": "net1", "public": false},
        {"id": "net2", "public": false},
        {"id": "net3", "public": true},
        {"id": "net4", "public": true}
    ],
    "ports": [
        {"id": "p1", "network": "net1"},
        {"id": "p2", "network": "net3"},
        {"id": "p3", "network": "net2"}
    ]
}
        "#;

        let policy = r#"
package example

default allow := false                              # unless otherwise defined, allow is false

allow := true {                                     # allow is true if...
    count(violation) == 0                           # there are zero violations.
}

xx := true {                                     # allow is true if...
    1 < 2                                           # there are zero violations.
}

violation[server.id] {                              # a server is in the violation set if...
    some server
    public_server[server]                           # it exists in the 'public_server' set and...
    server.protocols[_] == "http"                   # it contains the insecure "http" protocol.
}

violation[server.id] {                              # a server is in the violation set if...
    server := input.servers[_]                      # it exists in the input.servers collection and...
    server.protocols[_] == "telnet"                 # it contains the "telnet" protocol.
}

public_server[server] {                             # a server exists in the public_server set if...
    some i, j
    server := input.servers[_]                      # it exists in the input.servers collection and...
    server.ports[_] == input.ports[i].id            # it references a port in the input.ports collection and...
    input.ports[i].network == input.networks[j].id  # the port references a network in the input.networks collection and...
    input.networks[j].public                        # the network is public.
}
        "#;

        let _=  engine.add_input_json(Some(input.to_string()));
        engine.add_policy_from_string("regox.rego".to_string(),policy.to_string()).unwrap();

        let result = engine.eval_query("data.example.allow".to_string(), true);

        println!("{}", serde_json::to_string_pretty(&result.unwrap()).unwrap());

        let report = engine.engine.get_coverage_report();
        match report {
            Ok(report) => {
                println!("{}", "");
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    #[test]
    fn test_eval_rule() -> Result<()> {
        // Create an engine for evaluating Rego policies.
        let mut engine = RegoEngine::new();

        // Add policy to the engine.
        engine.add_policy(
            // Filename to be associated with the policy.
            "hello.rego".to_string(),

            // Rego policy that just sets a message.
            r#"
       package test
       message = "Hello, World!"
    "#.to_string()
        )?;

        // Evaluate the policy, fetch the message and print it.
        let results = engine.eval_query("data.test.message".to_string(), false)?;
        println!("{}", serde_json::to_string_pretty(&results)?);

        Ok(())
    }

    #[test]
    fn test_eval_xquery() -> Result<() > {
        let mut engine = Engine::new(false);

        engine.add_policy_from_string("hello.rego".to_string(),            r#"
           package test
           message = "Hello, World!"
        "#.to_string());

        // Evaluate the policy, fetch the message and print it.
        let results = engine.eval_query("data.test.message".to_string(), false)?;
        println!("{}", serde_json::to_string_pretty(&results)?);

        Ok(())
    }

    #[test]
    fn test_eval_coverage() -> Result<() > {
        let mut engine = Engine::new(true);

        engine.add_policy_from_string("hello.rego".to_string(),            r#"
           package test
           message = "Hello, World!"
        "#.to_string());

        // Evaluate the policy, fetch the message and print it.
        let results = engine.eval_query("data.test.message".to_string(),false)?;
        println!("{}", serde_json::to_string_pretty(&results)?);

        #[cfg(feature = "coverage")]
        let report = engine.engine.get_coverage_report()?;
        #[cfg(feature = "coverage")]
        println!("{}", report.to_colored_string()?);

        Ok(())
    }
}

