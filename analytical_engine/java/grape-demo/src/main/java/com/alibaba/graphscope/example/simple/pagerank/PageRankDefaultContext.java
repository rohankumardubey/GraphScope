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

package com.alibaba.graphscope.example.simple.pagerank;

import com.alibaba.fastffi.FFIByteString;
import com.alibaba.graphscope.app.DefaultContextBase;
import com.alibaba.graphscope.ds.Vertex;
import com.alibaba.graphscope.ds.VertexRange;
import com.alibaba.graphscope.fragment.ImmutableEdgecutFragment;
import com.alibaba.graphscope.parallel.DefaultMessageManager;
import com.alibaba.graphscope.stdcxx.StdVector;
import com.alibaba.graphscope.utils.DoubleArrayWrapper;
import com.alibaba.graphscope.utils.IntArrayWrapper;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;

public class PageRankDefaultContext implements DefaultContextBase<Long, Long, Long, Double> {
    public double alpha;
    public int maxIteration;
    public int superStep;
    public double danglingSum;

    public DoubleArrayWrapper pagerank;
    public IntArrayWrapper degree;

    @Override
    public void Init(
            ImmutableEdgecutFragment<Long, Long, Long, Double> frag,
            DefaultMessageManager messageManager,
            StdVector<FFIByteString> args) {
        alpha = Double.parseDouble(args.get(0).toString());
        maxIteration = Integer.parseInt(args.get(1).toString());
        System.out.println("alpha: [" + alpha + "], max iteration: [" + maxIteration + "]");
        pagerank = new DoubleArrayWrapper((frag.getVerticesNum().intValue()), 0.0);
        degree = new IntArrayWrapper((int) frag.getInnerVerticesNum().intValue(), 0);
    }

    @Override
    public void Output(ImmutableEdgecutFragment<Long, Long, Long, Double> frag) {
        String prefix = "/tmp/pagerank_output";
        String filePath = prefix + "_frag_" + String.valueOf(frag.fid());
        try {
            FileWriter fileWritter = new FileWriter(new File(filePath));
            BufferedWriter bufferedWriter = new BufferedWriter(fileWritter);
            VertexRange<Long> innerNodes = frag.innerVertices();

            Vertex<Long> cur = innerNodes.begin();
            for (long index = 0; index < frag.getInnerVerticesNum(); ++index) {
                cur.SetValue(index);
                Long oid = frag.getId(cur);
                bufferedWriter.write(
                        cur.GetValue() + "\t" + oid + "\t" + pagerank.get(index) + "\n");
            }
            bufferedWriter.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
