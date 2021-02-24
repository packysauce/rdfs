// I'mma slap some traits in here until protobuf codegen can do it for me
use paste::paste;

use crate::{HdfsServer::{VersionRequestProto, VersionResponseProto}, NamenodeProtocol::*};

macro_rules! async_rpc {
    (;) => {};
    (
        service $trait_name:ident {
            $(
                $(#[$outer:meta])*
                rpc $name:ident($req_type:ident) returns ($resp_type:ident);
            )+
        }
    ) => {
        paste! {
            #[async_trait::async_trait]
            pub trait $trait_name {
            $(
                $(#[$outer])*
                fn [<$name:snake>](req: $req_type) -> anyhow::Result<$resp_type>;
            )+
            }
        }
    };
}

async_rpc! {
service NamenodeProtocolService {
    /**
     * Get list of blocks for a given datanode with length
     * of blocks adding up to given size.
     */
    rpc getBlocks(GetBlocksRequestProto) returns(GetBlocksResponseProto);

    /**
     * Get the current block keys
     */
    rpc getBlockKeys(GetBlockKeysRequestProto) returns(GetBlockKeysResponseProto);

    /**
     * Get the transaction ID of the most recently persisted editlog record
     */
    rpc getTransactionId(GetTransactionIdRequestProto)
        returns(GetTransactionIdResponseProto);

    /**
     * Get the transaction ID of the most recently persisted editlog record
     */
    rpc getMostRecentCheckpointTxId(GetMostRecentCheckpointTxIdRequestProto)
        returns(GetMostRecentCheckpointTxIdResponseProto);

    /**
     * Close the current editlog and open a new one for checkpointing purposes
     */
    rpc rollEditLog(RollEditLogRequestProto) returns(RollEditLogResponseProto);

    /**
     * Request info about the version running on this NameNode
     */
    rpc versionRequest(VersionRequestProto) returns(VersionResponseProto);

    /**
     * Report from a sub-ordinate namenode of an error to the active namenode.
     * Active namenode may decide to unregister the reporting namenode
     * depending on the error.
     */
    rpc errorReport(ErrorReportRequestProto) returns(ErrorReportResponseProto);

    /**
     * Request to register a sub-ordinate namenode
     */
    rpc registerSubordinateNamenode(RegisterRequestProto) returns(RegisterResponseProto);

    /**
     * Request to start a checkpoint.
     */
    rpc startCheckpoint(StartCheckpointRequestProto)
        returns(StartCheckpointResponseProto);

    /**
     * End of finalize the previously started checkpoint
     */
    rpc endCheckpoint(EndCheckpointRequestProto)
        returns(EndCheckpointResponseProto);

    /**
     * Get editlog manifests from the active namenode for all the editlogs
     */
    rpc getEditLogManifest(GetEditLogManifestRequestProto)
        returns(GetEditLogManifestResponseProto);

    /**
     * Return whether the NameNode is in upgrade state (false) or not (true)
     */
    rpc isUpgradeFinalized(IsUpgradeFinalizedRequestProto)
        returns (IsUpgradeFinalizedResponseProto);

    /**
     * Return whether the NameNode is in rolling upgrade (true) or not (false).
     */
    rpc isRollingUpgrade(IsRollingUpgradeRequestProto)
        returns (IsRollingUpgradeResponseProto);

    /**
     * Return the sps path from namenode
     */
    rpc getNextSPSPath(GetNextSPSPathRequestProto)
        returns (GetNextSPSPathResponseProto);
    }
}
