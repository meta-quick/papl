package com.datasafe.rego;

import lombok.Data;

@Data
public class Location {
    /// Line number. Starts at 1.
    public int row;
    /// Column number. Starts at 1.
    public int col;

    @Override
    public String toString(){
        return String.format("{row:%s,col:%s}",row,col);
    }
}
