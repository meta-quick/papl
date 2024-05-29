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

package com.datasafe.cedar;

import com.datasafe.papl.Engine;
import com.datasafe.papl.EngineType;

/**
 * @author gaosg
 */
public class CedarEngine implements AutoCloseable {
    private final Engine engine;

    public CedarEngine() {
        engine = new Engine(EngineType.CEDAR);
    }

    public void addPolicy(String policy) {
        engine.cedarAddPolicy(policy);
    }

    public void addEntity(String entity){
        engine.cedarAddEntity(entity);
    }

    public String authorize(String principal, String action, String resource, String json_context){
        return  engine.cedarAuthorize(principal,action,resource,json_context);
    }

    public void createStore(String path){
        engine.newFileStore(path);
    }

    public void createMemoStore(){
        engine.newMemoryStore();
    }

    public void storeSave(String key,String value,String version) {
        engine.storeSave(key,value,version);
    }

    public void storeDelKey(String key) {
        engine.storeDeleteKey(key);
    }

    public String storeGet(String key){
        return engine.storeGetKey(key);
    }

    @Override
    public void close() throws Exception {
        engine.close();
    }
}
