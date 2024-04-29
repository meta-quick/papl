package com.datasafe;

import com.datasafe.papl.IStore;
import com.datasafe.papl.MemoryStore;
import com.datasafe.policy.FolderRegoPolicy;
import com.datasafe.policy.ResultSet;
import junit.framework.TestCase;

public class FolderRegoPolicyTest extends TestCase {
    public void test_policy(){
        IStore store = new MemoryStore();
        FolderRegoPolicy policy = new FolderRegoPolicy(store);

        policy.prepare("a/b/c.txt","package policy\ndefault allow := false\n");
        policy.prepare("a/b/","package policy\ndefault allow := true\n");

        ResultSet r = policy.eval("data.policy.allow","a/b/c.txt","{}");
        System.out.println(r);
        r = policy.eval("data.policy.allow","a/b/","{}");
        System.out.println(r);
        r = policy.eval("data.policy.list","a/b/","{}");
        System.out.println(r);
    }

    public void test_foldEval(){
        IStore store = new MemoryStore();
        FolderRegoPolicy policy = new FolderRegoPolicy(store);
        policy.prepare("a/b/c.txt","package policy\ndefault allow := false\n");
        policy.prepare("a/b/","package policy\ndefault list := true\n");
        ResultSet r = policy.foldEval("data.policy.list","a/b/c.txt","{}");
        System.out.println(r.toExpression()[0].value);
    }
}
