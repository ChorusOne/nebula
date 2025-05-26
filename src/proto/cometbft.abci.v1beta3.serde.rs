// @generated
impl serde::Serialize for CommitInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.round != 0 {
            len += 1;
        }
        if !self.votes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.CommitInfo", len)?;
        if self.round != 0 {
            struct_ser.serialize_field("round", &self.round)?;
        }
        if !self.votes.is_empty() {
            struct_ser.serialize_field("votes", &self.votes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommitInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "round",
            "votes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Round,
            Votes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "round" => Ok(GeneratedField::Round),
                            "votes" => Ok(GeneratedField::Votes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommitInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.CommitInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CommitInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut round__ = None;
                let mut votes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Round => {
                            if round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("round"));
                            }
                            round__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Votes => {
                            if votes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("votes"));
                            }
                            votes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CommitInfo {
                    round: round__.unwrap_or_default(),
                    votes: votes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.CommitInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecTxResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        if !self.log.is_empty() {
            len += 1;
        }
        if !self.info.is_empty() {
            len += 1;
        }
        if self.gas_wanted != 0 {
            len += 1;
        }
        if self.gas_used != 0 {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        if !self.codespace.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.ExecTxResult", len)?;
        if self.code != 0 {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if !self.log.is_empty() {
            struct_ser.serialize_field("log", &self.log)?;
        }
        if !self.info.is_empty() {
            struct_ser.serialize_field("info", &self.info)?;
        }
        if self.gas_wanted != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gas_wanted", ToString::to_string(&self.gas_wanted).as_str())?;
        }
        if self.gas_used != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gas_used", ToString::to_string(&self.gas_used).as_str())?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        if !self.codespace.is_empty() {
            struct_ser.serialize_field("codespace", &self.codespace)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecTxResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "data",
            "log",
            "info",
            "gas_wanted",
            "gas_used",
            "events",
            "codespace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Data,
            Log,
            Info,
            GasWanted,
            GasUsed,
            Events,
            Codespace,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "data" => Ok(GeneratedField::Data),
                            "log" => Ok(GeneratedField::Log),
                            "info" => Ok(GeneratedField::Info),
                            "gas_wanted" => Ok(GeneratedField::GasWanted),
                            "gas_used" => Ok(GeneratedField::GasUsed),
                            "events" => Ok(GeneratedField::Events),
                            "codespace" => Ok(GeneratedField::Codespace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecTxResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.ExecTxResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecTxResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut data__ = None;
                let mut log__ = None;
                let mut info__ = None;
                let mut gas_wanted__ = None;
                let mut gas_used__ = None;
                let mut events__ = None;
                let mut codespace__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Log => {
                            if log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("log"));
                            }
                            log__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GasWanted => {
                            if gas_wanted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gas_wanted"));
                            }
                            gas_wanted__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GasUsed => {
                            if gas_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gas_used"));
                            }
                            gas_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Codespace => {
                            if codespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codespace"));
                            }
                            codespace__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExecTxResult {
                    code: code__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                    log: log__.unwrap_or_default(),
                    info: info__.unwrap_or_default(),
                    gas_wanted: gas_wanted__.unwrap_or_default(),
                    gas_used: gas_used__.unwrap_or_default(),
                    events: events__.unwrap_or_default(),
                    codespace: codespace__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.ExecTxResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtendedCommitInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.round != 0 {
            len += 1;
        }
        if !self.votes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.ExtendedCommitInfo", len)?;
        if self.round != 0 {
            struct_ser.serialize_field("round", &self.round)?;
        }
        if !self.votes.is_empty() {
            struct_ser.serialize_field("votes", &self.votes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtendedCommitInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "round",
            "votes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Round,
            Votes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "round" => Ok(GeneratedField::Round),
                            "votes" => Ok(GeneratedField::Votes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtendedCommitInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.ExtendedCommitInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExtendedCommitInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut round__ = None;
                let mut votes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Round => {
                            if round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("round"));
                            }
                            round__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Votes => {
                            if votes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("votes"));
                            }
                            votes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExtendedCommitInfo {
                    round: round__.unwrap_or_default(),
                    votes: votes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.ExtendedCommitInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtendedVoteInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.validator.is_some() {
            len += 1;
        }
        if !self.vote_extension.is_empty() {
            len += 1;
        }
        if !self.extension_signature.is_empty() {
            len += 1;
        }
        if self.block_id_flag != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.ExtendedVoteInfo", len)?;
        if let Some(v) = self.validator.as_ref() {
            struct_ser.serialize_field("validator", v)?;
        }
        if !self.vote_extension.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("voteExtension", pbjson::private::base64::encode(&self.vote_extension).as_str())?;
        }
        if !self.extension_signature.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("extensionSignature", pbjson::private::base64::encode(&self.extension_signature).as_str())?;
        }
        if self.block_id_flag != 0 {
            let v = super::super::types::v1beta1::BlockIdFlag::try_from(self.block_id_flag)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.block_id_flag)))?;
            struct_ser.serialize_field("blockIdFlag", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtendedVoteInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "validator",
            "vote_extension",
            "voteExtension",
            "extension_signature",
            "extensionSignature",
            "block_id_flag",
            "blockIdFlag",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Validator,
            VoteExtension,
            ExtensionSignature,
            BlockIdFlag,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "validator" => Ok(GeneratedField::Validator),
                            "voteExtension" | "vote_extension" => Ok(GeneratedField::VoteExtension),
                            "extensionSignature" | "extension_signature" => Ok(GeneratedField::ExtensionSignature),
                            "blockIdFlag" | "block_id_flag" => Ok(GeneratedField::BlockIdFlag),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtendedVoteInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.ExtendedVoteInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExtendedVoteInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut validator__ = None;
                let mut vote_extension__ = None;
                let mut extension_signature__ = None;
                let mut block_id_flag__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Validator => {
                            if validator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validator"));
                            }
                            validator__ = map_.next_value()?;
                        }
                        GeneratedField::VoteExtension => {
                            if vote_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voteExtension"));
                            }
                            vote_extension__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExtensionSignature => {
                            if extension_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionSignature"));
                            }
                            extension_signature__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockIdFlag => {
                            if block_id_flag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockIdFlag"));
                            }
                            block_id_flag__ = Some(map_.next_value::<super::super::types::v1beta1::BlockIdFlag>()? as i32);
                        }
                    }
                }
                Ok(ExtendedVoteInfo {
                    validator: validator__,
                    vote_extension: vote_extension__.unwrap_or_default(),
                    extension_signature: extension_signature__.unwrap_or_default(),
                    block_id_flag: block_id_flag__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.ExtendedVoteInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Request {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.Request", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                request::Value::Echo(v) => {
                    struct_ser.serialize_field("echo", v)?;
                }
                request::Value::Flush(v) => {
                    struct_ser.serialize_field("flush", v)?;
                }
                request::Value::Info(v) => {
                    struct_ser.serialize_field("info", v)?;
                }
                request::Value::InitChain(v) => {
                    struct_ser.serialize_field("initChain", v)?;
                }
                request::Value::Query(v) => {
                    struct_ser.serialize_field("query", v)?;
                }
                request::Value::CheckTx(v) => {
                    struct_ser.serialize_field("checkTx", v)?;
                }
                request::Value::Commit(v) => {
                    struct_ser.serialize_field("commit", v)?;
                }
                request::Value::ListSnapshots(v) => {
                    struct_ser.serialize_field("listSnapshots", v)?;
                }
                request::Value::OfferSnapshot(v) => {
                    struct_ser.serialize_field("offerSnapshot", v)?;
                }
                request::Value::LoadSnapshotChunk(v) => {
                    struct_ser.serialize_field("loadSnapshotChunk", v)?;
                }
                request::Value::ApplySnapshotChunk(v) => {
                    struct_ser.serialize_field("applySnapshotChunk", v)?;
                }
                request::Value::PrepareProposal(v) => {
                    struct_ser.serialize_field("prepareProposal", v)?;
                }
                request::Value::ProcessProposal(v) => {
                    struct_ser.serialize_field("processProposal", v)?;
                }
                request::Value::ExtendVote(v) => {
                    struct_ser.serialize_field("extendVote", v)?;
                }
                request::Value::VerifyVoteExtension(v) => {
                    struct_ser.serialize_field("verifyVoteExtension", v)?;
                }
                request::Value::FinalizeBlock(v) => {
                    struct_ser.serialize_field("finalizeBlock", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Request {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "echo",
            "flush",
            "info",
            "init_chain",
            "initChain",
            "query",
            "check_tx",
            "checkTx",
            "commit",
            "list_snapshots",
            "listSnapshots",
            "offer_snapshot",
            "offerSnapshot",
            "load_snapshot_chunk",
            "loadSnapshotChunk",
            "apply_snapshot_chunk",
            "applySnapshotChunk",
            "prepare_proposal",
            "prepareProposal",
            "process_proposal",
            "processProposal",
            "extend_vote",
            "extendVote",
            "verify_vote_extension",
            "verifyVoteExtension",
            "finalize_block",
            "finalizeBlock",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Echo,
            Flush,
            Info,
            InitChain,
            Query,
            CheckTx,
            Commit,
            ListSnapshots,
            OfferSnapshot,
            LoadSnapshotChunk,
            ApplySnapshotChunk,
            PrepareProposal,
            ProcessProposal,
            ExtendVote,
            VerifyVoteExtension,
            FinalizeBlock,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "echo" => Ok(GeneratedField::Echo),
                            "flush" => Ok(GeneratedField::Flush),
                            "info" => Ok(GeneratedField::Info),
                            "initChain" | "init_chain" => Ok(GeneratedField::InitChain),
                            "query" => Ok(GeneratedField::Query),
                            "checkTx" | "check_tx" => Ok(GeneratedField::CheckTx),
                            "commit" => Ok(GeneratedField::Commit),
                            "listSnapshots" | "list_snapshots" => Ok(GeneratedField::ListSnapshots),
                            "offerSnapshot" | "offer_snapshot" => Ok(GeneratedField::OfferSnapshot),
                            "loadSnapshotChunk" | "load_snapshot_chunk" => Ok(GeneratedField::LoadSnapshotChunk),
                            "applySnapshotChunk" | "apply_snapshot_chunk" => Ok(GeneratedField::ApplySnapshotChunk),
                            "prepareProposal" | "prepare_proposal" => Ok(GeneratedField::PrepareProposal),
                            "processProposal" | "process_proposal" => Ok(GeneratedField::ProcessProposal),
                            "extendVote" | "extend_vote" => Ok(GeneratedField::ExtendVote),
                            "verifyVoteExtension" | "verify_vote_extension" => Ok(GeneratedField::VerifyVoteExtension),
                            "finalizeBlock" | "finalize_block" => Ok(GeneratedField::FinalizeBlock),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Request;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.Request")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Request, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Echo => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("echo"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::Echo)
;
                        }
                        GeneratedField::Flush => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flush"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::Flush)
;
                        }
                        GeneratedField::Info => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::Info)
;
                        }
                        GeneratedField::InitChain => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initChain"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::InitChain)
;
                        }
                        GeneratedField::Query => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::Query)
;
                        }
                        GeneratedField::CheckTx => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkTx"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::CheckTx)
;
                        }
                        GeneratedField::Commit => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commit"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::Commit)
;
                        }
                        GeneratedField::ListSnapshots => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listSnapshots"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::ListSnapshots)
;
                        }
                        GeneratedField::OfferSnapshot => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerSnapshot"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::OfferSnapshot)
;
                        }
                        GeneratedField::LoadSnapshotChunk => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadSnapshotChunk"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::LoadSnapshotChunk)
;
                        }
                        GeneratedField::ApplySnapshotChunk => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applySnapshotChunk"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::ApplySnapshotChunk)
;
                        }
                        GeneratedField::PrepareProposal => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prepareProposal"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::PrepareProposal)
;
                        }
                        GeneratedField::ProcessProposal => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("processProposal"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::ProcessProposal)
;
                        }
                        GeneratedField::ExtendVote => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extendVote"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::ExtendVote)
;
                        }
                        GeneratedField::VerifyVoteExtension => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifyVoteExtension"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::VerifyVoteExtension)
;
                        }
                        GeneratedField::FinalizeBlock => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finalizeBlock"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::FinalizeBlock)
;
                        }
                    }
                }
                Ok(Request {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.Request", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestExtendVote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if !self.txs.is_empty() {
            len += 1;
        }
        if self.proposed_last_commit.is_some() {
            len += 1;
        }
        if !self.misbehavior.is_empty() {
            len += 1;
        }
        if !self.next_validators_hash.is_empty() {
            len += 1;
        }
        if !self.proposer_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.RequestExtendVote", len)?;
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if !self.txs.is_empty() {
            struct_ser.serialize_field("txs", &self.txs.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.proposed_last_commit.as_ref() {
            struct_ser.serialize_field("proposedLastCommit", v)?;
        }
        if !self.misbehavior.is_empty() {
            struct_ser.serialize_field("misbehavior", &self.misbehavior)?;
        }
        if !self.next_validators_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nextValidatorsHash", pbjson::private::base64::encode(&self.next_validators_hash).as_str())?;
        }
        if !self.proposer_address.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("proposerAddress", pbjson::private::base64::encode(&self.proposer_address).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestExtendVote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "height",
            "time",
            "txs",
            "proposed_last_commit",
            "proposedLastCommit",
            "misbehavior",
            "next_validators_hash",
            "nextValidatorsHash",
            "proposer_address",
            "proposerAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Height,
            Time,
            Txs,
            ProposedLastCommit,
            Misbehavior,
            NextValidatorsHash,
            ProposerAddress,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            "height" => Ok(GeneratedField::Height),
                            "time" => Ok(GeneratedField::Time),
                            "txs" => Ok(GeneratedField::Txs),
                            "proposedLastCommit" | "proposed_last_commit" => Ok(GeneratedField::ProposedLastCommit),
                            "misbehavior" => Ok(GeneratedField::Misbehavior),
                            "nextValidatorsHash" | "next_validators_hash" => Ok(GeneratedField::NextValidatorsHash),
                            "proposerAddress" | "proposer_address" => Ok(GeneratedField::ProposerAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestExtendVote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.RequestExtendVote")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestExtendVote, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut height__ = None;
                let mut time__ = None;
                let mut txs__ = None;
                let mut proposed_last_commit__ = None;
                let mut misbehavior__ = None;
                let mut next_validators_hash__ = None;
                let mut proposer_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::Txs => {
                            if txs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txs"));
                            }
                            txs__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ProposedLastCommit => {
                            if proposed_last_commit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposedLastCommit"));
                            }
                            proposed_last_commit__ = map_.next_value()?;
                        }
                        GeneratedField::Misbehavior => {
                            if misbehavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("misbehavior"));
                            }
                            misbehavior__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextValidatorsHash => {
                            if next_validators_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextValidatorsHash"));
                            }
                            next_validators_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProposerAddress => {
                            if proposer_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposerAddress"));
                            }
                            proposer_address__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RequestExtendVote {
                    hash: hash__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    time: time__,
                    txs: txs__.unwrap_or_default(),
                    proposed_last_commit: proposed_last_commit__,
                    misbehavior: misbehavior__.unwrap_or_default(),
                    next_validators_hash: next_validators_hash__.unwrap_or_default(),
                    proposer_address: proposer_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.RequestExtendVote", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestFinalizeBlock {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.txs.is_empty() {
            len += 1;
        }
        if self.decided_last_commit.is_some() {
            len += 1;
        }
        if !self.misbehavior.is_empty() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if !self.next_validators_hash.is_empty() {
            len += 1;
        }
        if !self.proposer_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.RequestFinalizeBlock", len)?;
        if !self.txs.is_empty() {
            struct_ser.serialize_field("txs", &self.txs.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.decided_last_commit.as_ref() {
            struct_ser.serialize_field("decidedLastCommit", v)?;
        }
        if !self.misbehavior.is_empty() {
            struct_ser.serialize_field("misbehavior", &self.misbehavior)?;
        }
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if !self.next_validators_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nextValidatorsHash", pbjson::private::base64::encode(&self.next_validators_hash).as_str())?;
        }
        if !self.proposer_address.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("proposerAddress", pbjson::private::base64::encode(&self.proposer_address).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestFinalizeBlock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "txs",
            "decided_last_commit",
            "decidedLastCommit",
            "misbehavior",
            "hash",
            "height",
            "time",
            "next_validators_hash",
            "nextValidatorsHash",
            "proposer_address",
            "proposerAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Txs,
            DecidedLastCommit,
            Misbehavior,
            Hash,
            Height,
            Time,
            NextValidatorsHash,
            ProposerAddress,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "txs" => Ok(GeneratedField::Txs),
                            "decidedLastCommit" | "decided_last_commit" => Ok(GeneratedField::DecidedLastCommit),
                            "misbehavior" => Ok(GeneratedField::Misbehavior),
                            "hash" => Ok(GeneratedField::Hash),
                            "height" => Ok(GeneratedField::Height),
                            "time" => Ok(GeneratedField::Time),
                            "nextValidatorsHash" | "next_validators_hash" => Ok(GeneratedField::NextValidatorsHash),
                            "proposerAddress" | "proposer_address" => Ok(GeneratedField::ProposerAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestFinalizeBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.RequestFinalizeBlock")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestFinalizeBlock, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut txs__ = None;
                let mut decided_last_commit__ = None;
                let mut misbehavior__ = None;
                let mut hash__ = None;
                let mut height__ = None;
                let mut time__ = None;
                let mut next_validators_hash__ = None;
                let mut proposer_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Txs => {
                            if txs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txs"));
                            }
                            txs__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::DecidedLastCommit => {
                            if decided_last_commit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decidedLastCommit"));
                            }
                            decided_last_commit__ = map_.next_value()?;
                        }
                        GeneratedField::Misbehavior => {
                            if misbehavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("misbehavior"));
                            }
                            misbehavior__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::NextValidatorsHash => {
                            if next_validators_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextValidatorsHash"));
                            }
                            next_validators_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProposerAddress => {
                            if proposer_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposerAddress"));
                            }
                            proposer_address__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RequestFinalizeBlock {
                    txs: txs__.unwrap_or_default(),
                    decided_last_commit: decided_last_commit__,
                    misbehavior: misbehavior__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    time: time__,
                    next_validators_hash: next_validators_hash__.unwrap_or_default(),
                    proposer_address: proposer_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.RequestFinalizeBlock", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestInitChain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.time.is_some() {
            len += 1;
        }
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.consensus_params.is_some() {
            len += 1;
        }
        if !self.validators.is_empty() {
            len += 1;
        }
        if !self.app_state_bytes.is_empty() {
            len += 1;
        }
        if self.initial_height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.RequestInitChain", len)?;
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if let Some(v) = self.consensus_params.as_ref() {
            struct_ser.serialize_field("consensusParams", v)?;
        }
        if !self.validators.is_empty() {
            struct_ser.serialize_field("validators", &self.validators)?;
        }
        if !self.app_state_bytes.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("appStateBytes", pbjson::private::base64::encode(&self.app_state_bytes).as_str())?;
        }
        if self.initial_height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("initialHeight", ToString::to_string(&self.initial_height).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestInitChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "time",
            "chain_id",
            "chainId",
            "consensus_params",
            "consensusParams",
            "validators",
            "app_state_bytes",
            "appStateBytes",
            "initial_height",
            "initialHeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Time,
            ChainId,
            ConsensusParams,
            Validators,
            AppStateBytes,
            InitialHeight,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "time" => Ok(GeneratedField::Time),
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "consensusParams" | "consensus_params" => Ok(GeneratedField::ConsensusParams),
                            "validators" => Ok(GeneratedField::Validators),
                            "appStateBytes" | "app_state_bytes" => Ok(GeneratedField::AppStateBytes),
                            "initialHeight" | "initial_height" => Ok(GeneratedField::InitialHeight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestInitChain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.RequestInitChain")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestInitChain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut time__ = None;
                let mut chain_id__ = None;
                let mut consensus_params__ = None;
                let mut validators__ = None;
                let mut app_state_bytes__ = None;
                let mut initial_height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConsensusParams => {
                            if consensus_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusParams"));
                            }
                            consensus_params__ = map_.next_value()?;
                        }
                        GeneratedField::Validators => {
                            if validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validators"));
                            }
                            validators__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppStateBytes => {
                            if app_state_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appStateBytes"));
                            }
                            app_state_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::InitialHeight => {
                            if initial_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialHeight"));
                            }
                            initial_height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RequestInitChain {
                    time: time__,
                    chain_id: chain_id__.unwrap_or_default(),
                    consensus_params: consensus_params__,
                    validators: validators__.unwrap_or_default(),
                    app_state_bytes: app_state_bytes__.unwrap_or_default(),
                    initial_height: initial_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.RequestInitChain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestPrepareProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_tx_bytes != 0 {
            len += 1;
        }
        if !self.txs.is_empty() {
            len += 1;
        }
        if self.local_last_commit.is_some() {
            len += 1;
        }
        if !self.misbehavior.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if !self.next_validators_hash.is_empty() {
            len += 1;
        }
        if !self.proposer_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.RequestPrepareProposal", len)?;
        if self.max_tx_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxTxBytes", ToString::to_string(&self.max_tx_bytes).as_str())?;
        }
        if !self.txs.is_empty() {
            struct_ser.serialize_field("txs", &self.txs.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.local_last_commit.as_ref() {
            struct_ser.serialize_field("localLastCommit", v)?;
        }
        if !self.misbehavior.is_empty() {
            struct_ser.serialize_field("misbehavior", &self.misbehavior)?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if !self.next_validators_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nextValidatorsHash", pbjson::private::base64::encode(&self.next_validators_hash).as_str())?;
        }
        if !self.proposer_address.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("proposerAddress", pbjson::private::base64::encode(&self.proposer_address).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestPrepareProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_tx_bytes",
            "maxTxBytes",
            "txs",
            "local_last_commit",
            "localLastCommit",
            "misbehavior",
            "height",
            "time",
            "next_validators_hash",
            "nextValidatorsHash",
            "proposer_address",
            "proposerAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxTxBytes,
            Txs,
            LocalLastCommit,
            Misbehavior,
            Height,
            Time,
            NextValidatorsHash,
            ProposerAddress,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxTxBytes" | "max_tx_bytes" => Ok(GeneratedField::MaxTxBytes),
                            "txs" => Ok(GeneratedField::Txs),
                            "localLastCommit" | "local_last_commit" => Ok(GeneratedField::LocalLastCommit),
                            "misbehavior" => Ok(GeneratedField::Misbehavior),
                            "height" => Ok(GeneratedField::Height),
                            "time" => Ok(GeneratedField::Time),
                            "nextValidatorsHash" | "next_validators_hash" => Ok(GeneratedField::NextValidatorsHash),
                            "proposerAddress" | "proposer_address" => Ok(GeneratedField::ProposerAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestPrepareProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.RequestPrepareProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestPrepareProposal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_tx_bytes__ = None;
                let mut txs__ = None;
                let mut local_last_commit__ = None;
                let mut misbehavior__ = None;
                let mut height__ = None;
                let mut time__ = None;
                let mut next_validators_hash__ = None;
                let mut proposer_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxTxBytes => {
                            if max_tx_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTxBytes"));
                            }
                            max_tx_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Txs => {
                            if txs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txs"));
                            }
                            txs__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::LocalLastCommit => {
                            if local_last_commit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localLastCommit"));
                            }
                            local_last_commit__ = map_.next_value()?;
                        }
                        GeneratedField::Misbehavior => {
                            if misbehavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("misbehavior"));
                            }
                            misbehavior__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::NextValidatorsHash => {
                            if next_validators_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextValidatorsHash"));
                            }
                            next_validators_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProposerAddress => {
                            if proposer_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposerAddress"));
                            }
                            proposer_address__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RequestPrepareProposal {
                    max_tx_bytes: max_tx_bytes__.unwrap_or_default(),
                    txs: txs__.unwrap_or_default(),
                    local_last_commit: local_last_commit__,
                    misbehavior: misbehavior__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    time: time__,
                    next_validators_hash: next_validators_hash__.unwrap_or_default(),
                    proposer_address: proposer_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.RequestPrepareProposal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestProcessProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.txs.is_empty() {
            len += 1;
        }
        if self.proposed_last_commit.is_some() {
            len += 1;
        }
        if !self.misbehavior.is_empty() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if !self.next_validators_hash.is_empty() {
            len += 1;
        }
        if !self.proposer_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.RequestProcessProposal", len)?;
        if !self.txs.is_empty() {
            struct_ser.serialize_field("txs", &self.txs.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.proposed_last_commit.as_ref() {
            struct_ser.serialize_field("proposedLastCommit", v)?;
        }
        if !self.misbehavior.is_empty() {
            struct_ser.serialize_field("misbehavior", &self.misbehavior)?;
        }
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if !self.next_validators_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nextValidatorsHash", pbjson::private::base64::encode(&self.next_validators_hash).as_str())?;
        }
        if !self.proposer_address.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("proposerAddress", pbjson::private::base64::encode(&self.proposer_address).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestProcessProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "txs",
            "proposed_last_commit",
            "proposedLastCommit",
            "misbehavior",
            "hash",
            "height",
            "time",
            "next_validators_hash",
            "nextValidatorsHash",
            "proposer_address",
            "proposerAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Txs,
            ProposedLastCommit,
            Misbehavior,
            Hash,
            Height,
            Time,
            NextValidatorsHash,
            ProposerAddress,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "txs" => Ok(GeneratedField::Txs),
                            "proposedLastCommit" | "proposed_last_commit" => Ok(GeneratedField::ProposedLastCommit),
                            "misbehavior" => Ok(GeneratedField::Misbehavior),
                            "hash" => Ok(GeneratedField::Hash),
                            "height" => Ok(GeneratedField::Height),
                            "time" => Ok(GeneratedField::Time),
                            "nextValidatorsHash" | "next_validators_hash" => Ok(GeneratedField::NextValidatorsHash),
                            "proposerAddress" | "proposer_address" => Ok(GeneratedField::ProposerAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestProcessProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.RequestProcessProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestProcessProposal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut txs__ = None;
                let mut proposed_last_commit__ = None;
                let mut misbehavior__ = None;
                let mut hash__ = None;
                let mut height__ = None;
                let mut time__ = None;
                let mut next_validators_hash__ = None;
                let mut proposer_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Txs => {
                            if txs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txs"));
                            }
                            txs__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ProposedLastCommit => {
                            if proposed_last_commit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposedLastCommit"));
                            }
                            proposed_last_commit__ = map_.next_value()?;
                        }
                        GeneratedField::Misbehavior => {
                            if misbehavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("misbehavior"));
                            }
                            misbehavior__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::NextValidatorsHash => {
                            if next_validators_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextValidatorsHash"));
                            }
                            next_validators_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProposerAddress => {
                            if proposer_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposerAddress"));
                            }
                            proposer_address__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RequestProcessProposal {
                    txs: txs__.unwrap_or_default(),
                    proposed_last_commit: proposed_last_commit__,
                    misbehavior: misbehavior__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    time: time__,
                    next_validators_hash: next_validators_hash__.unwrap_or_default(),
                    proposer_address: proposer_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.RequestProcessProposal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestVerifyVoteExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if !self.vote_extension.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.RequestVerifyVoteExtension", len)?;
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if !self.validator_address.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("validatorAddress", pbjson::private::base64::encode(&self.validator_address).as_str())?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if !self.vote_extension.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("voteExtension", pbjson::private::base64::encode(&self.vote_extension).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestVerifyVoteExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "validator_address",
            "validatorAddress",
            "height",
            "vote_extension",
            "voteExtension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            ValidatorAddress,
            Height,
            VoteExtension,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            "validatorAddress" | "validator_address" => Ok(GeneratedField::ValidatorAddress),
                            "height" => Ok(GeneratedField::Height),
                            "voteExtension" | "vote_extension" => Ok(GeneratedField::VoteExtension),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestVerifyVoteExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.RequestVerifyVoteExtension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestVerifyVoteExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut validator_address__ = None;
                let mut height__ = None;
                let mut vote_extension__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorAddress"));
                            }
                            validator_address__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VoteExtension => {
                            if vote_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voteExtension"));
                            }
                            vote_extension__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RequestVerifyVoteExtension {
                    hash: hash__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    vote_extension: vote_extension__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.RequestVerifyVoteExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.Response", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                response::Value::Exception(v) => {
                    struct_ser.serialize_field("exception", v)?;
                }
                response::Value::Echo(v) => {
                    struct_ser.serialize_field("echo", v)?;
                }
                response::Value::Flush(v) => {
                    struct_ser.serialize_field("flush", v)?;
                }
                response::Value::Info(v) => {
                    struct_ser.serialize_field("info", v)?;
                }
                response::Value::InitChain(v) => {
                    struct_ser.serialize_field("initChain", v)?;
                }
                response::Value::Query(v) => {
                    struct_ser.serialize_field("query", v)?;
                }
                response::Value::CheckTx(v) => {
                    struct_ser.serialize_field("checkTx", v)?;
                }
                response::Value::Commit(v) => {
                    struct_ser.serialize_field("commit", v)?;
                }
                response::Value::ListSnapshots(v) => {
                    struct_ser.serialize_field("listSnapshots", v)?;
                }
                response::Value::OfferSnapshot(v) => {
                    struct_ser.serialize_field("offerSnapshot", v)?;
                }
                response::Value::LoadSnapshotChunk(v) => {
                    struct_ser.serialize_field("loadSnapshotChunk", v)?;
                }
                response::Value::ApplySnapshotChunk(v) => {
                    struct_ser.serialize_field("applySnapshotChunk", v)?;
                }
                response::Value::PrepareProposal(v) => {
                    struct_ser.serialize_field("prepareProposal", v)?;
                }
                response::Value::ProcessProposal(v) => {
                    struct_ser.serialize_field("processProposal", v)?;
                }
                response::Value::ExtendVote(v) => {
                    struct_ser.serialize_field("extendVote", v)?;
                }
                response::Value::VerifyVoteExtension(v) => {
                    struct_ser.serialize_field("verifyVoteExtension", v)?;
                }
                response::Value::FinalizeBlock(v) => {
                    struct_ser.serialize_field("finalizeBlock", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exception",
            "echo",
            "flush",
            "info",
            "init_chain",
            "initChain",
            "query",
            "check_tx",
            "checkTx",
            "commit",
            "list_snapshots",
            "listSnapshots",
            "offer_snapshot",
            "offerSnapshot",
            "load_snapshot_chunk",
            "loadSnapshotChunk",
            "apply_snapshot_chunk",
            "applySnapshotChunk",
            "prepare_proposal",
            "prepareProposal",
            "process_proposal",
            "processProposal",
            "extend_vote",
            "extendVote",
            "verify_vote_extension",
            "verifyVoteExtension",
            "finalize_block",
            "finalizeBlock",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Exception,
            Echo,
            Flush,
            Info,
            InitChain,
            Query,
            CheckTx,
            Commit,
            ListSnapshots,
            OfferSnapshot,
            LoadSnapshotChunk,
            ApplySnapshotChunk,
            PrepareProposal,
            ProcessProposal,
            ExtendVote,
            VerifyVoteExtension,
            FinalizeBlock,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "exception" => Ok(GeneratedField::Exception),
                            "echo" => Ok(GeneratedField::Echo),
                            "flush" => Ok(GeneratedField::Flush),
                            "info" => Ok(GeneratedField::Info),
                            "initChain" | "init_chain" => Ok(GeneratedField::InitChain),
                            "query" => Ok(GeneratedField::Query),
                            "checkTx" | "check_tx" => Ok(GeneratedField::CheckTx),
                            "commit" => Ok(GeneratedField::Commit),
                            "listSnapshots" | "list_snapshots" => Ok(GeneratedField::ListSnapshots),
                            "offerSnapshot" | "offer_snapshot" => Ok(GeneratedField::OfferSnapshot),
                            "loadSnapshotChunk" | "load_snapshot_chunk" => Ok(GeneratedField::LoadSnapshotChunk),
                            "applySnapshotChunk" | "apply_snapshot_chunk" => Ok(GeneratedField::ApplySnapshotChunk),
                            "prepareProposal" | "prepare_proposal" => Ok(GeneratedField::PrepareProposal),
                            "processProposal" | "process_proposal" => Ok(GeneratedField::ProcessProposal),
                            "extendVote" | "extend_vote" => Ok(GeneratedField::ExtendVote),
                            "verifyVoteExtension" | "verify_vote_extension" => Ok(GeneratedField::VerifyVoteExtension),
                            "finalizeBlock" | "finalize_block" => Ok(GeneratedField::FinalizeBlock),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.Response")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Exception => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exception"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::Exception)
;
                        }
                        GeneratedField::Echo => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("echo"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::Echo)
;
                        }
                        GeneratedField::Flush => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flush"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::Flush)
;
                        }
                        GeneratedField::Info => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::Info)
;
                        }
                        GeneratedField::InitChain => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initChain"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::InitChain)
;
                        }
                        GeneratedField::Query => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::Query)
;
                        }
                        GeneratedField::CheckTx => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkTx"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::CheckTx)
;
                        }
                        GeneratedField::Commit => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commit"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::Commit)
;
                        }
                        GeneratedField::ListSnapshots => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listSnapshots"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::ListSnapshots)
;
                        }
                        GeneratedField::OfferSnapshot => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offerSnapshot"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::OfferSnapshot)
;
                        }
                        GeneratedField::LoadSnapshotChunk => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadSnapshotChunk"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::LoadSnapshotChunk)
;
                        }
                        GeneratedField::ApplySnapshotChunk => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applySnapshotChunk"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::ApplySnapshotChunk)
;
                        }
                        GeneratedField::PrepareProposal => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prepareProposal"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::PrepareProposal)
;
                        }
                        GeneratedField::ProcessProposal => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("processProposal"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::ProcessProposal)
;
                        }
                        GeneratedField::ExtendVote => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extendVote"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::ExtendVote)
;
                        }
                        GeneratedField::VerifyVoteExtension => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifyVoteExtension"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::VerifyVoteExtension)
;
                        }
                        GeneratedField::FinalizeBlock => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finalizeBlock"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::FinalizeBlock)
;
                        }
                    }
                }
                Ok(Response {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseCheckTx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        if !self.log.is_empty() {
            len += 1;
        }
        if !self.info.is_empty() {
            len += 1;
        }
        if self.gas_wanted != 0 {
            len += 1;
        }
        if self.gas_used != 0 {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        if !self.codespace.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.ResponseCheckTx", len)?;
        if self.code != 0 {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if !self.log.is_empty() {
            struct_ser.serialize_field("log", &self.log)?;
        }
        if !self.info.is_empty() {
            struct_ser.serialize_field("info", &self.info)?;
        }
        if self.gas_wanted != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gas_wanted", ToString::to_string(&self.gas_wanted).as_str())?;
        }
        if self.gas_used != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gas_used", ToString::to_string(&self.gas_used).as_str())?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        if !self.codespace.is_empty() {
            struct_ser.serialize_field("codespace", &self.codespace)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseCheckTx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "data",
            "log",
            "info",
            "gas_wanted",
            "gas_used",
            "events",
            "codespace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Data,
            Log,
            Info,
            GasWanted,
            GasUsed,
            Events,
            Codespace,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "data" => Ok(GeneratedField::Data),
                            "log" => Ok(GeneratedField::Log),
                            "info" => Ok(GeneratedField::Info),
                            "gas_wanted" => Ok(GeneratedField::GasWanted),
                            "gas_used" => Ok(GeneratedField::GasUsed),
                            "events" => Ok(GeneratedField::Events),
                            "codespace" => Ok(GeneratedField::Codespace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseCheckTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.ResponseCheckTx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseCheckTx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut data__ = None;
                let mut log__ = None;
                let mut info__ = None;
                let mut gas_wanted__ = None;
                let mut gas_used__ = None;
                let mut events__ = None;
                let mut codespace__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Log => {
                            if log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("log"));
                            }
                            log__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GasWanted => {
                            if gas_wanted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gas_wanted"));
                            }
                            gas_wanted__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GasUsed => {
                            if gas_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gas_used"));
                            }
                            gas_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Codespace => {
                            if codespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codespace"));
                            }
                            codespace__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResponseCheckTx {
                    code: code__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                    log: log__.unwrap_or_default(),
                    info: info__.unwrap_or_default(),
                    gas_wanted: gas_wanted__.unwrap_or_default(),
                    gas_used: gas_used__.unwrap_or_default(),
                    events: events__.unwrap_or_default(),
                    codespace: codespace__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.ResponseCheckTx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseCommit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.retain_height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.ResponseCommit", len)?;
        if self.retain_height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("retainHeight", ToString::to_string(&self.retain_height).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseCommit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "retain_height",
            "retainHeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RetainHeight,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "retainHeight" | "retain_height" => Ok(GeneratedField::RetainHeight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseCommit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.ResponseCommit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseCommit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut retain_height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RetainHeight => {
                            if retain_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retainHeight"));
                            }
                            retain_height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ResponseCommit {
                    retain_height: retain_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.ResponseCommit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseExtendVote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vote_extension.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.ResponseExtendVote", len)?;
        if !self.vote_extension.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("voteExtension", pbjson::private::base64::encode(&self.vote_extension).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseExtendVote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vote_extension",
            "voteExtension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VoteExtension,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "voteExtension" | "vote_extension" => Ok(GeneratedField::VoteExtension),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseExtendVote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.ResponseExtendVote")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseExtendVote, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vote_extension__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VoteExtension => {
                            if vote_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voteExtension"));
                            }
                            vote_extension__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ResponseExtendVote {
                    vote_extension: vote_extension__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.ResponseExtendVote", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseFinalizeBlock {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.events.is_empty() {
            len += 1;
        }
        if !self.tx_results.is_empty() {
            len += 1;
        }
        if !self.validator_updates.is_empty() {
            len += 1;
        }
        if self.consensus_param_updates.is_some() {
            len += 1;
        }
        if !self.app_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.ResponseFinalizeBlock", len)?;
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        if !self.tx_results.is_empty() {
            struct_ser.serialize_field("txResults", &self.tx_results)?;
        }
        if !self.validator_updates.is_empty() {
            struct_ser.serialize_field("validatorUpdates", &self.validator_updates)?;
        }
        if let Some(v) = self.consensus_param_updates.as_ref() {
            struct_ser.serialize_field("consensusParamUpdates", v)?;
        }
        if !self.app_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("appHash", pbjson::private::base64::encode(&self.app_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseFinalizeBlock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "events",
            "tx_results",
            "txResults",
            "validator_updates",
            "validatorUpdates",
            "consensus_param_updates",
            "consensusParamUpdates",
            "app_hash",
            "appHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Events,
            TxResults,
            ValidatorUpdates,
            ConsensusParamUpdates,
            AppHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "events" => Ok(GeneratedField::Events),
                            "txResults" | "tx_results" => Ok(GeneratedField::TxResults),
                            "validatorUpdates" | "validator_updates" => Ok(GeneratedField::ValidatorUpdates),
                            "consensusParamUpdates" | "consensus_param_updates" => Ok(GeneratedField::ConsensusParamUpdates),
                            "appHash" | "app_hash" => Ok(GeneratedField::AppHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseFinalizeBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.ResponseFinalizeBlock")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseFinalizeBlock, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut events__ = None;
                let mut tx_results__ = None;
                let mut validator_updates__ = None;
                let mut consensus_param_updates__ = None;
                let mut app_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TxResults => {
                            if tx_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txResults"));
                            }
                            tx_results__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidatorUpdates => {
                            if validator_updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorUpdates"));
                            }
                            validator_updates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConsensusParamUpdates => {
                            if consensus_param_updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusParamUpdates"));
                            }
                            consensus_param_updates__ = map_.next_value()?;
                        }
                        GeneratedField::AppHash => {
                            if app_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appHash"));
                            }
                            app_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ResponseFinalizeBlock {
                    events: events__.unwrap_or_default(),
                    tx_results: tx_results__.unwrap_or_default(),
                    validator_updates: validator_updates__.unwrap_or_default(),
                    consensus_param_updates: consensus_param_updates__,
                    app_hash: app_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.ResponseFinalizeBlock", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseInitChain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consensus_params.is_some() {
            len += 1;
        }
        if !self.validators.is_empty() {
            len += 1;
        }
        if !self.app_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.ResponseInitChain", len)?;
        if let Some(v) = self.consensus_params.as_ref() {
            struct_ser.serialize_field("consensusParams", v)?;
        }
        if !self.validators.is_empty() {
            struct_ser.serialize_field("validators", &self.validators)?;
        }
        if !self.app_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("appHash", pbjson::private::base64::encode(&self.app_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseInitChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consensus_params",
            "consensusParams",
            "validators",
            "app_hash",
            "appHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConsensusParams,
            Validators,
            AppHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "consensusParams" | "consensus_params" => Ok(GeneratedField::ConsensusParams),
                            "validators" => Ok(GeneratedField::Validators),
                            "appHash" | "app_hash" => Ok(GeneratedField::AppHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseInitChain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.ResponseInitChain")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseInitChain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consensus_params__ = None;
                let mut validators__ = None;
                let mut app_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConsensusParams => {
                            if consensus_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusParams"));
                            }
                            consensus_params__ = map_.next_value()?;
                        }
                        GeneratedField::Validators => {
                            if validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validators"));
                            }
                            validators__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppHash => {
                            if app_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appHash"));
                            }
                            app_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ResponseInitChain {
                    consensus_params: consensus_params__,
                    validators: validators__.unwrap_or_default(),
                    app_hash: app_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.ResponseInitChain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseVerifyVoteExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.ResponseVerifyVoteExtension", len)?;
        if self.status != 0 {
            let v = response_verify_vote_extension::VerifyStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseVerifyVoteExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseVerifyVoteExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.ResponseVerifyVoteExtension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseVerifyVoteExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<response_verify_vote_extension::VerifyStatus>()? as i32);
                        }
                    }
                }
                Ok(ResponseVerifyVoteExtension {
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.ResponseVerifyVoteExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for response_verify_vote_extension::VerifyStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Accept => "ACCEPT",
            Self::Reject => "REJECT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for response_verify_vote_extension::VerifyStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "ACCEPT",
            "REJECT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = response_verify_vote_extension::VerifyStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(response_verify_vote_extension::VerifyStatus::Unknown),
                    "ACCEPT" => Ok(response_verify_vote_extension::VerifyStatus::Accept),
                    "REJECT" => Ok(response_verify_vote_extension::VerifyStatus::Reject),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TxResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if !self.tx.is_empty() {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.TxResult", len)?;
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", &self.index)?;
        }
        if !self.tx.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("tx", pbjson::private::base64::encode(&self.tx).as_str())?;
        }
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
            "index",
            "tx",
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
            Index,
            Tx,
            Result,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "height" => Ok(GeneratedField::Height),
                            "index" => Ok(GeneratedField::Index),
                            "tx" => Ok(GeneratedField::Tx),
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.TxResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                let mut index__ = None;
                let mut tx__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Tx => {
                            if tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tx"));
                            }
                            tx__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TxResult {
                    height: height__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    tx: tx__.unwrap_or_default(),
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.TxResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VoteInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.validator.is_some() {
            len += 1;
        }
        if self.block_id_flag != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta3.VoteInfo", len)?;
        if let Some(v) = self.validator.as_ref() {
            struct_ser.serialize_field("validator", v)?;
        }
        if self.block_id_flag != 0 {
            let v = super::super::types::v1beta1::BlockIdFlag::try_from(self.block_id_flag)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.block_id_flag)))?;
            struct_ser.serialize_field("blockIdFlag", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VoteInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "validator",
            "block_id_flag",
            "blockIdFlag",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Validator,
            BlockIdFlag,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "validator" => Ok(GeneratedField::Validator),
                            "blockIdFlag" | "block_id_flag" => Ok(GeneratedField::BlockIdFlag),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VoteInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta3.VoteInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VoteInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut validator__ = None;
                let mut block_id_flag__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Validator => {
                            if validator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validator"));
                            }
                            validator__ = map_.next_value()?;
                        }
                        GeneratedField::BlockIdFlag => {
                            if block_id_flag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockIdFlag"));
                            }
                            block_id_flag__ = Some(map_.next_value::<super::super::types::v1beta1::BlockIdFlag>()? as i32);
                        }
                    }
                }
                Ok(VoteInfo {
                    validator: validator__,
                    block_id_flag: block_id_flag__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta3.VoteInfo", FIELDS, GeneratedVisitor)
    }
}
