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

package com.datasafe.rego;

import com.datasafe.papl.Engine;
import com.datasafe.papl.EngineType;
import com.google.gson.Gson;

import java.util.Map;
import java.util.TreeMap;

public class RegoEngine implements AutoCloseable{
    private final Engine engine;

    public RegoEngine(){
        engine = new Engine(EngineType.REGO);
    }

    @Override
    public void close() throws Exception {
        engine.close();
    }

    public void addPolicyFile(String fname){
        engine.regoAddPolicyFile(fname);
    }

    public void addPolicy(String path,String rego){
        engine.regoAddPolicy(path,rego);
    }

    public void addDataFile(String path){
        engine.regoAddDataFile(path);
    }

    public void addJSONData(String json){
        engine.regoAddStringData(json);
    }

    public void setInputFile(String path){
        engine.regoAddDataFile(path);
    }

    public void setJSONInput(String json){
        engine.regoAddInputJSON(json);
    }

    public void addBundles(String[] folders){
        engine.regoAddBundles(folders);
    }

    public void clearData(){
        engine.regoClearData();
    }

    public String evalPureQuery(String query){
        return engine.regoEvalQuery(query);
    }

    public TreeMap evalMapQuery(String query){
        String result = engine.regoEvalQuery(query);
        Gson gson = new Gson();
        return gson.fromJson(result, TreeMap.class);
    }

    public QueryResults evalQuery(String query){
        String result = engine.regoEvalQuery(query);
        System.out.println(result);
        Gson gson = new Gson();
        QueryResults results = gson.fromJson(result, QueryResults.class);
        return results;
    }
}
