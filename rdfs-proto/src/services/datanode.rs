/*
service DatanodeProtocolService {
  /**
   * Register a datanode at a namenode
   */
  rpc registerDatanode(RegisterDatanodeRequestProto)
      returns(RegisterDatanodeResponseProto);

  /**
   * Send heartbeat from datanode to namenode
   */
  rpc sendHeartbeat(HeartbeatRequestProto) returns(HeartbeatResponseProto);

  /**
   * Report blocks at a given datanode to the namenode
   */
  rpc blockReport(BlockReportRequestProto) returns(BlockReportResponseProto);

  /**
   * Report cached blocks at a datanode to the namenode
   */
  rpc cacheReport(CacheReportRequestProto) returns(CacheReportResponseProto);

  /**
   * Incremental block report from the DN. This contains info about recently
   * received and deleted blocks, as well as when blocks start being
   * received.
   */
  rpc blockReceivedAndDeleted(BlockReceivedAndDeletedRequestProto) 
      returns(BlockReceivedAndDeletedResponseProto);

  /**
   * Report from a datanode of an error to the active namenode.
   * Used for debugging.
   */
  rpc errorReport(ErrorReportRequestProto) returns(ErrorReportResponseProto);
  
  /**
   * Request the version
   */
  rpc versionRequest(VersionRequestProto) returns(VersionResponseProto);

  /**
   * Report corrupt blocks at the specified location
   */
  rpc reportBadBlocks(ReportBadBlocksRequestProto) returns(ReportBadBlocksResponseProto);

  /**
   * Commit block synchronization during lease recovery.
   */
  rpc commitBlockSynchronization(CommitBlockSynchronizationRequestProto)
      returns(CommitBlockSynchronizationResponseProto);
}
*/