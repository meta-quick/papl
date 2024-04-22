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

package com.datasafe.papl;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;
import java.util.concurrent.locks.ReentrantLock;

/**
 * PAPL Engine.
 * @author gaosg
 */
public class Engine implements AutoCloseable {
    // Methods exposed from Rust side, you can run
    // `javac -h . src/main/java/com/datasafe/papl/Engine.java` to update
    // expected native header at `bindings/java/com_datasafe_pap_Engine.h`
    // if you update the native API.


    private static native long nativeNewRegoEngine();
    private static native long nativeCloseRegoEngine(long enginePtr);
    private static native void nativeRegoAddPolicy(long enginePtr, String path, String rego);
    private static native void nativeRegoAddBundles(long enginePtr, String[] bundles);
    private static native void nativeRegoAddInput(long enginePtr, String input);
    private static native void nativeRegoAddInputJSON(long enginePtr, String json);
    private static native void nativeRegoAddData(long enginePtr, String path);
    private static native void nativeRegoAddStringData(long enginePtr, String data);
    private static native void nativeRegoAddPolicyFile(long enginePtr, String path);
    private static native void nativeRegoClearData(long enginePtr);
    private static native String nativeRegoEvalQuery(long enginePtr, String query);
    private static native String nativeCedarAddPolicy(long enginePtr, String policy);
    private static native String nativeCedarAddEntity(long enginePtr, String json_entity);
    private static native String nativeCedarAuthorize(long enginePtr, String principal,String action,String resource,String json_context);



    private static native long nativeNewCedarEngine();
    private static native long nativeCloseCedarEngine(long enginePtr);


    private ReentrantLock mutex = new ReentrantLock();

    private long enginePtr = 0;
    private EngineType type = EngineType.REGO;

    public Engine(){

    }

    public Engine(EngineType type){
        initialize(type);
    }

    public void initialize(EngineType type){
        try{
            mutex.lock();
            if(type == EngineType.CEDAR){
                enginePtr = nativeNewCedarEngine();
            }else {
                enginePtr =  nativeNewRegoEngine();
            }
            this.type = type;
        }finally {
            mutex.unlock();
        }

    }

    public void regoAddPolicy(String path, String rego){
        nativeRegoAddPolicy(this.enginePtr,path,rego);
    }

    public void regoAddBundles(String[] bundles){
        nativeRegoAddBundles(this.enginePtr,bundles);
    }

    public void regoAddInput(String input){
        nativeRegoAddInput(this.enginePtr,input);
    }

    public void regoAddInputJSON(String json){
        nativeRegoAddInputJSON(this.enginePtr,json);
    }

    public void regoAddDataFile(String path){
        nativeRegoAddData(enginePtr,path);
    }

    public void regoAddStringData(String data){
        nativeRegoAddStringData(enginePtr,data);
    }

    public void regoAddPolicyFile(String path){
        nativeRegoAddPolicyFile(enginePtr,path);
    }

    public void regoClearData(){
        nativeRegoClearData(enginePtr);
    }

    public String regoEvalQuery(String query){
        return nativeRegoEvalQuery(enginePtr,query);
    }

    public void cedarAddPolicy(String polciy) {
        nativeCedarAddPolicy(enginePtr,polciy);
    }

    public void cedarAddEntity(String json) {
        nativeCedarAddEntity(enginePtr,json);
    }

    public String cedarAuthorize(String principal,String action,String resource,String json_context) {
       return nativeCedarAuthorize(enginePtr,principal,action,resource,json_context);
    }

    @Override
    public void close() throws Exception {
        if (enginePtr == 0) {
            return;
        }
        try{
            mutex.lock();
            if(type == EngineType.CEDAR){
                nativeCloseCedarEngine(enginePtr);
            }else {
                nativeCloseRegoEngine(enginePtr);
            }
            enginePtr = 0;
        }finally {
            mutex.unlock();
        }
    }

    static {
        final StringBuilder targetTripleBuilder = new StringBuilder();

        final String arch = System.getProperty("os.arch").toLowerCase();
        if (arch.equals("aarch64")) {
            targetTripleBuilder.append("aarch64");
        } else {
            targetTripleBuilder.append("x86_64");
        }
        targetTripleBuilder.append("-");

        final String os = System.getProperty("os.name").toLowerCase();
        if (os.startsWith("windows")) {
            targetTripleBuilder.append("pc-windows-msvc");
        } else if (os.startsWith("mac")) {
            targetTripleBuilder.append("apple-darwin");
        } else {
            targetTripleBuilder.append("unknown-linux-gnu");
        }

        loadNativeLibrary(targetTripleBuilder.toString());
    }

    private static void loadNativeLibrary(String targetTriple) {
        try {
            // try dynamic library - the search path can be configured via "-Djava.library.path"
            System.loadLibrary("papl_java");
            return;
        } catch (UnsatisfiedLinkError ignore) {
            // ignore - try from classpath
        }

        // Native libraries will be bundles into JARs like:
        // `aarch64-apple-darwin/libpapl_java.dylib`
        final String libraryName = System.mapLibraryName("papl_java");
        final String libraryPath = "/" + targetTriple + "/" + libraryName;

        try (final InputStream is = Engine.class.getResourceAsStream(libraryPath)) {
            if (is == null) {
                throw new RuntimeException("Cannot find " + libraryPath);
            }
            final int dot = libraryPath.indexOf('.');
            final File tmpFile = File.createTempFile(libraryPath.substring(0, dot), libraryPath.substring(dot));
            tmpFile.deleteOnExit();
            Files.copy(is, tmpFile.toPath(), StandardCopyOption.REPLACE_EXISTING);
            System.load(tmpFile.getAbsolutePath());
        } catch (IOException exception) {
            throw new RuntimeException(exception);
        }
    }
}