/*
 * Copyright 2021 Alibaba Group Holding Limited.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *  	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.alibaba.graphscope.stdcxx;

import static com.alibaba.graphscope.utils.CppHeaderName.CORE_JAVA_TYPE_ALIAS_H;
import static com.alibaba.graphscope.utils.JNILibraryName.JNI_LIBRARY_NAME;

import com.alibaba.fastffi.CXXHead;
import com.alibaba.fastffi.CXXOperator;
import com.alibaba.fastffi.CXXPointer;
import com.alibaba.fastffi.CXXReference;
import com.alibaba.fastffi.CXXValue;
import com.alibaba.fastffi.FFIFactory;
import com.alibaba.fastffi.FFIGen;
import com.alibaba.fastffi.FFITypeAlias;

@FFIGen(library = JNI_LIBRARY_NAME)
@CXXHead(system = {"vector", "string"})
@CXXHead(CORE_JAVA_TYPE_ALIAS_H)
@FFITypeAlias("std::vector")
// @CXXTemplate(cxx = "jshort", java = "java.lang.Short")
// @CXXTemplate(cxx = "jint", java = "java.lang.Integer")
// @CXXTemplate(cxx = "jchar", java = "java.lang.Character")
// @CXXTemplate(cxx = "jbyte", java = "java.lang.Byte")
// @CXXTemplate(cxx = "jlong", java = "java.lang.Long")
// @CXXTemplate(cxx = "jdouble", java = "java.lang.Double")
// @CXXTemplate(cxx = "jfloat", java = "java.lang.Float")
// @CXXTemplate(cxx = "jboolean", java = "java.lang.Boolean")
// @CXXTemplate(cxx = "int64_t", java = "java.lang.Long")
// @CXXTemplate(cxx = "char", java = "java.lang.Byte")
// @CXXTemplate(cxx = "std::string", java = "com.alibaba.fastffi.FFIByteString")
// @CXXTemplate(cxx = "std::vector<char>", java =
// "com.alibaba.graphscope.stdcxx.StdVector<java.lang.Byte>")
// @CXXTemplate(
// cxx = "std::vector<int64_t>",
// java = "com.alibaba.graphscope.stdcxx.StdVector<java.lang.Long>")
//// @CXXTemplate(
// cxx = "std::vector<jint>",
// java = "com.alibaba.graphscope.stdcxx.StdVector<java.lang.Integer>")
//// @CXXTemplate(
// cxx = "std::vector<std::string>",
// java =
// "com.alibaba.graphscope.stdcxx.StdVector<com.alibaba.fastffi.FFIByteString>")
//// @CXXTemplate(
// cxx = GS_VERTEX_ARRAY + "<double>",
// java = "com.alibaba.graphscope.ds.GSVertexArray<java.lang.Double>")
public interface StdVector<E> extends CXXPointer {
    long size();

    @CXXOperator("[]")
    @CXXReference
    E get(long index);

    @CXXOperator("[]")
    void set(long index, @CXXReference E value);

    void push_back(@CXXValue E e);

    default void add(@CXXReference E value) {
        long size = size();
        long cap = capacity();
        if (size == cap) {
            reserve(cap << 1);
        }
        push_back(value);
    }

    default @CXXReference E append() {
        long size = size();
        long cap = capacity();
        if (size == cap) {
            reserve(cap << 1);
        }
        resize(size + 1);
        return get(size);
    }

    void clear();

    long data();

    long capacity();

    void reserve(long size);

    void resize(long size);

    @FFIFactory
    interface Factory<E> {
        StdVector<E> create();
    }
}
