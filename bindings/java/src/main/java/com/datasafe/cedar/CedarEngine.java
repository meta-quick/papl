package com.datasafe.cedar;

import com.datasafe.papl.Engine;
import com.datasafe.papl.EngineType;

/**
 * @author gaosg
 */
public class CedarEngine {
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
}
