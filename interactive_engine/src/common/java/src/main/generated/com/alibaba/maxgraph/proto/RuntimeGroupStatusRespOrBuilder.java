// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: data.proto

package com.alibaba.maxgraph.proto;

public interface RuntimeGroupStatusRespOrBuilder extends
    // @@protoc_insertion_point(interface_extends:RuntimeGroupStatusResp)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>optional .Response response = 1;</code>
   */
  boolean hasResponse();
  /**
   * <code>optional .Response response = 1;</code>
   */
  com.alibaba.maxgraph.proto.Response getResponse();
  /**
   * <code>optional .Response response = 1;</code>
   */
  com.alibaba.maxgraph.proto.ResponseOrBuilder getResponseOrBuilder();

  /**
   * <code>map&lt;int32, .RuntimeGroupStatusResp.Status&gt; runtimeGroupsStstus = 2;</code>
   */
  int getRuntimeGroupsStstusCount();
  /**
   * <code>map&lt;int32, .RuntimeGroupStatusResp.Status&gt; runtimeGroupsStstus = 2;</code>
   */
  boolean containsRuntimeGroupsStstus(
      int key);
  /**
   * Use {@link #getRuntimeGroupsStstusMap()} instead.
   */
  @java.lang.Deprecated
  java.util.Map<java.lang.Integer, com.alibaba.maxgraph.proto.RuntimeGroupStatusResp.Status>
  getRuntimeGroupsStstus();
  /**
   * <code>map&lt;int32, .RuntimeGroupStatusResp.Status&gt; runtimeGroupsStstus = 2;</code>
   */
  java.util.Map<java.lang.Integer, com.alibaba.maxgraph.proto.RuntimeGroupStatusResp.Status>
  getRuntimeGroupsStstusMap();
  /**
   * <code>map&lt;int32, .RuntimeGroupStatusResp.Status&gt; runtimeGroupsStstus = 2;</code>
   */
  com.alibaba.maxgraph.proto.RuntimeGroupStatusResp.Status getRuntimeGroupsStstusOrDefault(
      int key,
      com.alibaba.maxgraph.proto.RuntimeGroupStatusResp.Status defaultValue);
  /**
   * <code>map&lt;int32, .RuntimeGroupStatusResp.Status&gt; runtimeGroupsStstus = 2;</code>
   */
  com.alibaba.maxgraph.proto.RuntimeGroupStatusResp.Status getRuntimeGroupsStstusOrThrow(
      int key);
  /**
   * Use {@link #getRuntimeGroupsStstusValueMap()} instead.
   */
  @java.lang.Deprecated
  java.util.Map<java.lang.Integer, java.lang.Integer>
  getRuntimeGroupsStstusValue();
  /**
   * <code>map&lt;int32, .RuntimeGroupStatusResp.Status&gt; runtimeGroupsStstus = 2;</code>
   */
  java.util.Map<java.lang.Integer, java.lang.Integer>
  getRuntimeGroupsStstusValueMap();
  /**
   * <code>map&lt;int32, .RuntimeGroupStatusResp.Status&gt; runtimeGroupsStstus = 2;</code>
   */

  int getRuntimeGroupsStstusValueOrDefault(
      int key,
      int defaultValue);
  /**
   * <code>map&lt;int32, .RuntimeGroupStatusResp.Status&gt; runtimeGroupsStstus = 2;</code>
   */

  int getRuntimeGroupsStstusValueOrThrow(
      int key);
}