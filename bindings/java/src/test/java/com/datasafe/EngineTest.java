/*
 * Copyright 2024 brian <gao.brian@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.datasafe;

import java.util.Map;
import java.util.ArrayList;

import com.datasafe.papl.Engine;
import com.datasafe.papl.EngineType;
import com.datasafe.rego.QueryResults;
import com.datasafe.rego.RegoEngine;
import junit.framework.TestCase;
import com.google.gson.Gson;

/**
 * @author gaosg
 * @author gaosg
 */
public class EngineTest extends TestCase
{
    public void test_engine_natives() throws Exception {
        String path = "hello.rego";
        String pol = "package test\nmessage = concat(\", \", [input.message, data.message])";

        try (Engine engine = new Engine(EngineType.REGO) ){
            engine.regoAddPolicy(path,pol);
            String[] bundles = {"examples"};
            engine.regoAddBundles(bundles);
            engine.regoAddInput("examples/input.json");
            engine.regoAddInputJSON("{}");
            engine.regoAddDataFile("examples/input.json");
            engine.regoAddPolicyFile("examples/example.rego");
            engine.regoClearData();
        }
    }

    public void test_rego_query() throws Exception {
        String path = "hello.rego";
        String pol = "package test\nmessage = concat(\", \", [input.message, data.message])";

        String resJson;
        try (Engine engine = new Engine(EngineType.REGO) ){
            engine.regoAddPolicy(path,pol);
            engine.regoAddStringData("{\"message\":\"World!\"}");
            engine.regoAddInputJSON("{\"message\":\"Hello\"}");
            resJson = engine.regoEvalQuery("data.test.message");
            System.out.println(resJson);

            Gson gson = new Gson();
            Map res = gson.fromJson(resJson, Map.class);
            ArrayList results = (ArrayList) res.get("result");
            ArrayList expressions = (ArrayList) ((Map) results.get(0)).get("expressions");
            Map expression = (Map) expressions.get(0);
            assertEquals("Hello, World!", expression.get("value"));
        }
    }

    public void test_rego_engine() throws Exception {
        String path = "hello.rego";
        String pol = "package test\nmessage = concat(\", \", [input.message, data.message])";

        try (RegoEngine engine = new RegoEngine() ){
            engine.addPolicy(path,pol);
            engine.addJSONData("{\"message\":\"World!\"}");
            engine.setJSONInput("{\"message\":\"Hello\"}");
            QueryResults result = engine.evalQuery("data.test.message");
            System.out.println(result);
        }
    }

    public void test_rego_engine_exa() throws Exception {
        String path = "hello.rego";
        String pol = "package test\nmessage = 1>2";

        try (RegoEngine engine = new RegoEngine() ){
            engine.addPolicy(path,pol);
            engine.addJSONData("{\"message\":\"World!\"}");
            engine.setJSONInput("{\"message\":\"Hello\"}");
            QueryResults result = engine.evalQuery("data.test.message");
            System.out.println(result);
        }
    }
}

