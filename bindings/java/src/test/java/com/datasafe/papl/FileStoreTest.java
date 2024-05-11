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

import org.junit.After;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;

import java.io.File;

public class FileStoreTest {

    private String testFileName;
    private String testFileContent;

    @Before
    public void setUp() {
        testFileName = "testFile.txt";
        testFileContent = "This is a test file content";
    }

    @After
    public void tearDown() {
        File file = new File(testFileName);
        if (file.exists()) {
            file.delete();
        }
    }

    @Test
    public void testEnsureParentFolderExists_WithNonExistentParentFolder() {
        Assert.assertTrue(FileStore.ensureParentFolderExists(testFileName));
        Assert.assertFalse(new File(testFileName).exists());
    }

    @Test
    public void testEnsureParentFolderExists_WithExistingParentFolder() {
        File parentDir = new File(testFileName + ".d");
        parentDir.mkdir();
        Assert.assertTrue(FileStore.ensureParentFolderExists(testFileName));
        Assert.assertFalse(new File(testFileName).exists());

        parentDir.delete(); // Clean up
    }

    @Test
    public void testEnsureParentFolderExists_WithFileWithExistingParent() {
        File file = new File(testFileName);
        try {
            file.createNewFile();
            Assert.assertTrue(FileStore.ensureParentFolderExists(testFileName));
            Assert.assertTrue(file.exists());
        } catch (Exception e) {
            Assert.fail("Exception should not have been thrown for valid file creation");
        } finally {
            file.delete(); // Clean up
        }
    }

    @Test
    public void testEnsureParentFolderExists_WithNullFilePath() {
        Assert.assertFalse(FileStore.ensureParentFolderExists(null));
    }

    @Test
    public void testEnsureParentFolderExists_WithEmptyFilePath() {
        Assert.assertFalse(FileStore.ensureParentFolderExists(""));
    }
}
