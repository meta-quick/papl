package com.datasafe.rego;

import cn.hutool.json.JSONObject;

import java.util.Objects;
import java.util.Optional;

public class Result {
    public Expression[] expressions;

    public JSONObject bindings;
}
