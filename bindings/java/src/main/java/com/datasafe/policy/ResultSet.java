package com.datasafe.policy;

import com.datasafe.rego.Expression;
import com.datasafe.rego.QueryResults;

/**
 * @author gaosg
 */
public class ResultSet {
    public ResultStatus status;
    public QueryResults result;

    public Expression[] toExpression()
    {
        return this.result.getResult()[0].expressions;
    }
}
