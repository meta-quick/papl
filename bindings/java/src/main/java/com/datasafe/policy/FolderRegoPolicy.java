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

package com.datasafe.policy;

import cn.hutool.cache.CacheUtil;
import cn.hutool.cache.impl.LRUCache;
import com.datasafe.papl.IStore;
import com.datasafe.rego.QueryResults;
import com.datasafe.rego.RegoEngine;

import java.nio.file.Path;
import java.nio.file.Paths;

/**
 * @author gaosg
 */
public class FolderRegoPolicy {

    private IStore store = null;
    private LRUCache<String, RegoEngine> ENGINE_INSTANCE_CACHE = CacheUtil.newLRUCache(1000);

    public FolderRegoPolicy(IStore store){
        this.store = store;
    }

    public void prepare(String path,String policy){
        store.save(path,policy);
    }

    public void prepareData(String path,String policy){
        store.save(path,policy);
    }

    /**
     * Eval a query per given input
     * @param query  a string like data.policy.[action]
     * @param path
     * @param inputJSON
     * @return
     */
    public ResultSet eval(String query, String path, String inputJSON,String data){
        //find if engine cached
        RegoEngine engine = ENGINE_INSTANCE_CACHE.get(path);
        if (engine == null) {
            String policy = store.get(path);
            if (policy == null || policy.isEmpty()){
                ResultSet result = new ResultSet();
                result.status = ResultStatus.FAIL;
                return result;
            }

            //initial and cache engine
            engine = new RegoEngine();
            engine.addPolicy(path,policy);
            engine.clearData();
            if (data != null && !data.isEmpty()){
                engine.addJSONData(data);
            }

            ENGINE_INSTANCE_CACHE.put(path,engine);
        }

        engine.setJSONInput(inputJSON);
        QueryResults result = engine.evalQuery(query);
        ResultSet resultset = new ResultSet();
        resultset.status = ResultStatus.OK;
        resultset.result = result;

        return resultset;
    }

    public ResultSet foldEval(String query,String path,String input,String jsonData){
        //Create path from given path
        Path parent = Paths.get(path);

        boolean isDecided = false;
        while (!isDecided){
            ResultSet rs = eval(query,path,input,jsonData);
            if (rs.status == ResultStatus.OK){
                if (rs.result == null || rs.result.getResult() == null) {
                    parent = parent.getParent();
                    if (parent == null){
                        break;
                    }

                    path = parent.toString() + "/";
                    continue;
                }
                isDecided = true;
                return rs;
            } else { // No policy found
                //need check parent here
                parent = parent.getParent();
                if (parent == null){
                    break;
                }
                path = parent.toString() + "/";
            }
        }

        ResultSet result = new ResultSet();
        result.status = ResultStatus.FAIL;
        result.result = null;

        return result;
    }
}
