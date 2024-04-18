package com.datasafe.rego;

import cn.hutool.json.JSONObject;
import lombok.Data;

@Data
public class Expression {
        /// Computed value of the expression.
        public Object value;

        /// The Rego expression.
        public String text;

        /// Location of the expression in the query string.
        public Location location;
}
