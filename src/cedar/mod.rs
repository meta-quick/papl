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

use anyhow::{Result};
use std::cell::RefCell;
use cedar_policy::*;

pub struct Engine {
    pub policy: PolicySet,
    pub authorizer: Authorizer,
    pub entities: RefCell<Entities>
}


impl Engine {
    pub fn new() -> Self {
       Engine{
           policy: PolicySet::new(),
           authorizer: Authorizer::new(),
           entities: RefCell::new(Entities::empty()),
       }
    }

    pub fn create_request(&self, principal: String, action: String, resource: String, json_context: String) -> Result<Request> {
        let action = action.trim().to_string().parse().unwrap();
        let principal = principal.trim().to_string().parse().unwrap();
        let resource = resource.trim().to_string().parse().unwrap();
        let mut context = Context::empty();

        if !json_context.is_empty() {
           let json = json_context.trim().to_string();
           context = Context::from_json_str(&json,None).unwrap();
        }

        let request = Request::new(Some(principal), Some(action), Some(resource), context, None).unwrap();
        Ok(request)
    }

    pub fn add_policy(&mut self, policy: String) -> Result<()> {
        let policy  = policy.parse().unwrap();
        self.policy.add(policy);
        Ok(())
    }

    pub fn add_entity(&mut self, json: &str) -> Result<()> {
        let entities = Entities::empty().add_entities_from_json_str(json,None);
        self.entities.replace(entities.unwrap());
        Ok(())
    }


    pub fn authorize_request(&self, principal: String, action: String, resource: String, json_context: String) -> Response {
        let request = self.create_request(principal, action, resource, json_context).unwrap();
        self.authorize(request)
    }

    pub fn decide_request(&self, principal: String, action: String, resource: String, json_context: String) -> Decision {
        let request = self.create_request(principal, action, resource, json_context).unwrap();
        self.decide(request)
    }

    pub fn authorize(&self, request: Request) -> Response {
        let answer = self.authorizer.is_authorized(&request, &self.policy, &self.entities.borrow());
        answer
    }
    pub fn decide(&self, request: Request) -> Decision {
        let answer = self.authorizer.is_authorized(&request, &self.policy, &self.entities.borrow());
        answer.decision()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{bail, Ok, Result};
    use cedar_policy::*;

    #[test]
    fn test_add_policy() -> Result<()> {
        let mut cedar = Engine::new();

        const POLICY_SRC: &str = r#"
permit(principal == User::"alice", action == Action::"view", resource == File::"93");
"#;
        cedar.add_policy(POLICY_SRC.to_string());
        let action = r#"Action::"view""#.parse().unwrap();
        let alice = r#"User::"alice""#.parse().unwrap();
        let file = r#"File::"931""#.parse().unwrap();

        let request = Request::new(Some(alice), Some(action), Some(file), Context::empty(), None).unwrap();

        let answer = cedar.authorize(request);

        println!("{:?}", answer.decision());

        assert_eq!(answer.decision(), Decision::Deny);
        Ok(())
    }


    #[test]
    fn it_works() -> Result<()> {
        const POLICY_SRC: &str = r#"
permit(principal == User::"alice", action == Action::"view", resource == File::"93");
"#;
        let policy: PolicySet = POLICY_SRC.parse().unwrap();

        let action = r#"Action::"view""#.parse().unwrap();
        let alice = r#"User::"alice""#.parse().unwrap();
        let file = r#"File::"93""#.parse().unwrap();
        let request = Request::new(Some(alice), Some(action), Some(file), Context::empty(), None).unwrap();

        let entities = Entities::empty();
        let authorizer = Authorizer::new();
        let answer = authorizer.is_authorized(&request, &policy, &entities);

        // Should output `Allow`
        println!("{:?}", answer.decision());

        let action = r#"Action::"view""#.parse().unwrap();
        let bob = r#"User::"bob""#.parse().unwrap();
        let file = r#"File::"93""#.parse().unwrap();
        let request = Request::new(Some(bob), Some(action), Some(file), Context::empty(), None).unwrap();

        let answer = authorizer.is_authorized(&request, &policy, &entities);

        // Should output `Deny`
        println!("{:?}", answer.decision());
        Ok(())
    }
}