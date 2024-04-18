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

use anyhow::Result;
use jni::objects::{JClass, JObject, JString,JObjectArray};
use jni::sys::{jlong, jstring};
use jni::JNIEnv;
use papl::*;

fn throw_err<T>(mut env: JNIEnv, mut f: impl FnMut(&mut JNIEnv) -> Result<T>) -> Result<T> {
    match f(&mut env) {
        Ok(val) => Ok(val),
        Err(err) => {
            env.throw(err.to_string())?;
            Err(err)
        }
    }
}


#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeNewCedarEngine(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let engine = CedarEngine::new();
    Box::into_raw(Box::new(engine)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeCloseCedarEngine(
    _env : JNIEnv,
    _class: JClass,
    _engine: jlong,
) -> jlong {
    println!("Closing Cedar Engine {}", _engine);
    unsafe {
        let _engine = Box::from_raw(_engine as *mut CedarEngine);
    }
    0
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeNewRegoEngine(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let engine = RegoEngine::new();
    Box::into_raw(Box::new(engine)) as jlong
}


#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeCloseRegoEngine(
    _env : JNIEnv,
    _class: JClass,
    _engine: jlong,
) -> jlong {
    println!("Closing Rego Engine {}", _engine);
    unsafe {
        let _engine = Box::from_raw(_engine as *mut RegoEngine);
    }
    0
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddPolicy<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    path: JString<'local>,
    rego: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let path: String = env.get_string(&path)?.into();
        let rego: String = env.get_string(&rego)?.into();
        engine.add_policy_from_string(path,rego)?;
        Ok(())
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddBundles<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    bundles: JObjectArray<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let len = env.get_array_length(&bundles).unwrap();

        let mut local_bundles = Vec::<String>::with_capacity(len as usize);
        for i in 0..len {
            let bundle: JString = env.get_object_array_element(&bundles,i).unwrap().into();
            let bundle_path: String = env.get_string(&bundle)?.into();
            local_bundles.push(bundle_path);
        }
        engine.add_bundles(&local_bundles.as_slice())
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddInput<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    input: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let input: String = env.get_string(&input)?.into();
        println!("Adding input: {}", input);
        engine.add_input(Some(input))
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddInputJSON<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    json: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let json: String = env.get_string(&json)?.into();
        engine.add_input_json(Some(json))
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddData<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    path: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let path: String = env.get_string(&path)?.into();
        engine.add_data(Some(path))
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddStringData<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    data: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let data: String = env.get_string(&data)?.into();
        engine.add_data_from_string(data)
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoAddPolicyFile<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    path: JString<'local>,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let path: String = env.get_string(&path)?.into();
        engine.add_policy_from_file(path)
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoClearData<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
) {
    let _ = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        engine.clear_data();
        Ok(())
    });
}

#[no_mangle]
pub extern "system" fn Java_com_datasafe_papl_Engine_nativeRegoEvalQuery<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    query: JString<'local>,
) -> jstring {
    let res = throw_err(env, |env| {
        let engine = unsafe { &mut *(engine_ptr as *mut RegoEngine) };
        let query: String = env.get_string(&query)?.into();
        let results = engine.eval_query(query, false)?;
        let output = env.new_string(serde_json::to_string(&results)?)?;
        Ok(output.into_raw())
    });

    match res {
        Ok(val) => val,
        Err(_) => JObject::null().into_raw(),
    }
}




