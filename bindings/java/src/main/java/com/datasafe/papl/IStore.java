package com.datasafe.papl;

/**
 * @author gaosg
 */
public interface IStore extends AutoCloseable {
    public void save(String key,String value);
    public void delete(String key);
    public String get(String key);
}
