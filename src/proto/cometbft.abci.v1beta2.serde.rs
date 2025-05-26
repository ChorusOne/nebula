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
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.CommitInfo", len)?;
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
                formatter.write_str("struct cometbft.abci.v1beta2.CommitInfo")
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
        deserializer.deserialize_struct("cometbft.abci.v1beta2.CommitInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Event {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.attributes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.Event", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.attributes.is_empty() {
            struct_ser.serialize_field("attributes", &self.attributes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Event {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Attributes,
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
                            "type" => Ok(GeneratedField::Type),
                            "attributes" => Ok(GeneratedField::Attributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Event;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta2.Event")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Event, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut attributes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Event {
                    r#type: r#type__.unwrap_or_default(),
                    attributes: attributes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.Event", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventAttribute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if self.index {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.EventAttribute", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if self.index {
            struct_ser.serialize_field("index", &self.index)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventAttribute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
            "index",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
            Index,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            "index" => Ok(GeneratedField::Index),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventAttribute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta2.EventAttribute")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventAttribute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                let mut index__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventAttribute {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.EventAttribute", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.ExtendedCommitInfo", len)?;
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
                formatter.write_str("struct cometbft.abci.v1beta2.ExtendedCommitInfo")
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
        deserializer.deserialize_struct("cometbft.abci.v1beta2.ExtendedCommitInfo", FIELDS, GeneratedVisitor)
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
        if self.signed_last_block {
            len += 1;
        }
        if !self.vote_extension.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.ExtendedVoteInfo", len)?;
        if let Some(v) = self.validator.as_ref() {
            struct_ser.serialize_field("validator", v)?;
        }
        if self.signed_last_block {
            struct_ser.serialize_field("signedLastBlock", &self.signed_last_block)?;
        }
        if !self.vote_extension.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("voteExtension", pbjson::private::base64::encode(&self.vote_extension).as_str())?;
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
            "signed_last_block",
            "signedLastBlock",
            "vote_extension",
            "voteExtension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Validator,
            SignedLastBlock,
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
                            "validator" => Ok(GeneratedField::Validator),
                            "signedLastBlock" | "signed_last_block" => Ok(GeneratedField::SignedLastBlock),
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
            type Value = ExtendedVoteInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta2.ExtendedVoteInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExtendedVoteInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut validator__ = None;
                let mut signed_last_block__ = None;
                let mut vote_extension__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Validator => {
                            if validator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validator"));
                            }
                            validator__ = map_.next_value()?;
                        }
                        GeneratedField::SignedLastBlock => {
                            if signed_last_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signedLastBlock"));
                            }
                            signed_last_block__ = Some(map_.next_value()?);
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
                Ok(ExtendedVoteInfo {
                    validator: validator__,
                    signed_last_block: signed_last_block__.unwrap_or_default(),
                    vote_extension: vote_extension__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.ExtendedVoteInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Misbehavior {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if self.validator.is_some() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if self.total_voting_power != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.Misbehavior", len)?;
        if self.r#type != 0 {
            let v = MisbehaviorType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.validator.as_ref() {
            struct_ser.serialize_field("validator", v)?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if self.total_voting_power != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalVotingPower", ToString::to_string(&self.total_voting_power).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Misbehavior {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "validator",
            "height",
            "time",
            "total_voting_power",
            "totalVotingPower",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Validator,
            Height,
            Time,
            TotalVotingPower,
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
                            "type" => Ok(GeneratedField::Type),
                            "validator" => Ok(GeneratedField::Validator),
                            "height" => Ok(GeneratedField::Height),
                            "time" => Ok(GeneratedField::Time),
                            "totalVotingPower" | "total_voting_power" => Ok(GeneratedField::TotalVotingPower),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Misbehavior;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta2.Misbehavior")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Misbehavior, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut validator__ = None;
                let mut height__ = None;
                let mut time__ = None;
                let mut total_voting_power__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<MisbehaviorType>()? as i32);
                        }
                        GeneratedField::Validator => {
                            if validator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validator"));
                            }
                            validator__ = map_.next_value()?;
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
                        GeneratedField::TotalVotingPower => {
                            if total_voting_power__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalVotingPower"));
                            }
                            total_voting_power__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Misbehavior {
                    r#type: r#type__.unwrap_or_default(),
                    validator: validator__,
                    height: height__.unwrap_or_default(),
                    time: time__,
                    total_voting_power: total_voting_power__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.Misbehavior", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MisbehaviorType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::DuplicateVote => "DUPLICATE_VOTE",
            Self::LightClientAttack => "LIGHT_CLIENT_ATTACK",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MisbehaviorType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "DUPLICATE_VOTE",
            "LIGHT_CLIENT_ATTACK",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MisbehaviorType;

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
                    "UNKNOWN" => Ok(MisbehaviorType::Unknown),
                    "DUPLICATE_VOTE" => Ok(MisbehaviorType::DuplicateVote),
                    "LIGHT_CLIENT_ATTACK" => Ok(MisbehaviorType::LightClientAttack),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.Request", len)?;
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
                request::Value::BeginBlock(v) => {
                    struct_ser.serialize_field("beginBlock", v)?;
                }
                request::Value::CheckTx(v) => {
                    struct_ser.serialize_field("checkTx", v)?;
                }
                request::Value::DeliverTx(v) => {
                    struct_ser.serialize_field("deliverTx", v)?;
                }
                request::Value::EndBlock(v) => {
                    struct_ser.serialize_field("endBlock", v)?;
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
            "begin_block",
            "beginBlock",
            "check_tx",
            "checkTx",
            "deliver_tx",
            "deliverTx",
            "end_block",
            "endBlock",
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Echo,
            Flush,
            Info,
            InitChain,
            Query,
            BeginBlock,
            CheckTx,
            DeliverTx,
            EndBlock,
            Commit,
            ListSnapshots,
            OfferSnapshot,
            LoadSnapshotChunk,
            ApplySnapshotChunk,
            PrepareProposal,
            ProcessProposal,
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
                            "beginBlock" | "begin_block" => Ok(GeneratedField::BeginBlock),
                            "checkTx" | "check_tx" => Ok(GeneratedField::CheckTx),
                            "deliverTx" | "deliver_tx" => Ok(GeneratedField::DeliverTx),
                            "endBlock" | "end_block" => Ok(GeneratedField::EndBlock),
                            "commit" => Ok(GeneratedField::Commit),
                            "listSnapshots" | "list_snapshots" => Ok(GeneratedField::ListSnapshots),
                            "offerSnapshot" | "offer_snapshot" => Ok(GeneratedField::OfferSnapshot),
                            "loadSnapshotChunk" | "load_snapshot_chunk" => Ok(GeneratedField::LoadSnapshotChunk),
                            "applySnapshotChunk" | "apply_snapshot_chunk" => Ok(GeneratedField::ApplySnapshotChunk),
                            "prepareProposal" | "prepare_proposal" => Ok(GeneratedField::PrepareProposal),
                            "processProposal" | "process_proposal" => Ok(GeneratedField::ProcessProposal),
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
                formatter.write_str("struct cometbft.abci.v1beta2.Request")
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
                        GeneratedField::BeginBlock => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("beginBlock"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::BeginBlock)
;
                        }
                        GeneratedField::CheckTx => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkTx"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::CheckTx)
;
                        }
                        GeneratedField::DeliverTx => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deliverTx"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::DeliverTx)
;
                        }
                        GeneratedField::EndBlock => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endBlock"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(request::Value::EndBlock)
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
                    }
                }
                Ok(Request {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.Request", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestBeginBlock {
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
        if self.header.is_some() {
            len += 1;
        }
        if self.last_commit_info.is_some() {
            len += 1;
        }
        if !self.byzantine_validators.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.RequestBeginBlock", len)?;
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.last_commit_info.as_ref() {
            struct_ser.serialize_field("lastCommitInfo", v)?;
        }
        if !self.byzantine_validators.is_empty() {
            struct_ser.serialize_field("byzantineValidators", &self.byzantine_validators)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestBeginBlock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "header",
            "last_commit_info",
            "lastCommitInfo",
            "byzantine_validators",
            "byzantineValidators",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Header,
            LastCommitInfo,
            ByzantineValidators,
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
                            "header" => Ok(GeneratedField::Header),
                            "lastCommitInfo" | "last_commit_info" => Ok(GeneratedField::LastCommitInfo),
                            "byzantineValidators" | "byzantine_validators" => Ok(GeneratedField::ByzantineValidators),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestBeginBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta2.RequestBeginBlock")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestBeginBlock, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut header__ = None;
                let mut last_commit_info__ = None;
                let mut byzantine_validators__ = None;
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
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::LastCommitInfo => {
                            if last_commit_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastCommitInfo"));
                            }
                            last_commit_info__ = map_.next_value()?;
                        }
                        GeneratedField::ByzantineValidators => {
                            if byzantine_validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("byzantineValidators"));
                            }
                            byzantine_validators__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RequestBeginBlock {
                    hash: hash__.unwrap_or_default(),
                    header: header__,
                    last_commit_info: last_commit_info__,
                    byzantine_validators: byzantine_validators__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.RequestBeginBlock", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        if self.block_version != 0 {
            len += 1;
        }
        if self.p2p_version != 0 {
            len += 1;
        }
        if !self.abci_version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.RequestInfo", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if self.block_version != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("blockVersion", ToString::to_string(&self.block_version).as_str())?;
        }
        if self.p2p_version != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("p2pVersion", ToString::to_string(&self.p2p_version).as_str())?;
        }
        if !self.abci_version.is_empty() {
            struct_ser.serialize_field("abciVersion", &self.abci_version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "block_version",
            "blockVersion",
            "p2p_version",
            "p2pVersion",
            "abci_version",
            "abciVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            BlockVersion,
            P2pVersion,
            AbciVersion,
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
                            "version" => Ok(GeneratedField::Version),
                            "blockVersion" | "block_version" => Ok(GeneratedField::BlockVersion),
                            "p2pVersion" | "p2p_version" => Ok(GeneratedField::P2pVersion),
                            "abciVersion" | "abci_version" => Ok(GeneratedField::AbciVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta2.RequestInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut block_version__ = None;
                let mut p2p_version__ = None;
                let mut abci_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlockVersion => {
                            if block_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockVersion"));
                            }
                            block_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::P2pVersion => {
                            if p2p_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("p2pVersion"));
                            }
                            p2p_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AbciVersion => {
                            if abci_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abciVersion"));
                            }
                            abci_version__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RequestInfo {
                    version: version__.unwrap_or_default(),
                    block_version: block_version__.unwrap_or_default(),
                    p2p_version: p2p_version__.unwrap_or_default(),
                    abci_version: abci_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.RequestInfo", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.RequestInitChain", len)?;
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
                formatter.write_str("struct cometbft.abci.v1beta2.RequestInitChain")
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
        deserializer.deserialize_struct("cometbft.abci.v1beta2.RequestInitChain", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.RequestPrepareProposal", len)?;
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
                formatter.write_str("struct cometbft.abci.v1beta2.RequestPrepareProposal")
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
        deserializer.deserialize_struct("cometbft.abci.v1beta2.RequestPrepareProposal", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.RequestProcessProposal", len)?;
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
                formatter.write_str("struct cometbft.abci.v1beta2.RequestProcessProposal")
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
        deserializer.deserialize_struct("cometbft.abci.v1beta2.RequestProcessProposal", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.Response", len)?;
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
                response::Value::BeginBlock(v) => {
                    struct_ser.serialize_field("beginBlock", v)?;
                }
                response::Value::CheckTx(v) => {
                    struct_ser.serialize_field("checkTx", v)?;
                }
                response::Value::DeliverTx(v) => {
                    struct_ser.serialize_field("deliverTx", v)?;
                }
                response::Value::EndBlock(v) => {
                    struct_ser.serialize_field("endBlock", v)?;
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
            "begin_block",
            "beginBlock",
            "check_tx",
            "checkTx",
            "deliver_tx",
            "deliverTx",
            "end_block",
            "endBlock",
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Exception,
            Echo,
            Flush,
            Info,
            InitChain,
            Query,
            BeginBlock,
            CheckTx,
            DeliverTx,
            EndBlock,
            Commit,
            ListSnapshots,
            OfferSnapshot,
            LoadSnapshotChunk,
            ApplySnapshotChunk,
            PrepareProposal,
            ProcessProposal,
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
                            "beginBlock" | "begin_block" => Ok(GeneratedField::BeginBlock),
                            "checkTx" | "check_tx" => Ok(GeneratedField::CheckTx),
                            "deliverTx" | "deliver_tx" => Ok(GeneratedField::DeliverTx),
                            "endBlock" | "end_block" => Ok(GeneratedField::EndBlock),
                            "commit" => Ok(GeneratedField::Commit),
                            "listSnapshots" | "list_snapshots" => Ok(GeneratedField::ListSnapshots),
                            "offerSnapshot" | "offer_snapshot" => Ok(GeneratedField::OfferSnapshot),
                            "loadSnapshotChunk" | "load_snapshot_chunk" => Ok(GeneratedField::LoadSnapshotChunk),
                            "applySnapshotChunk" | "apply_snapshot_chunk" => Ok(GeneratedField::ApplySnapshotChunk),
                            "prepareProposal" | "prepare_proposal" => Ok(GeneratedField::PrepareProposal),
                            "processProposal" | "process_proposal" => Ok(GeneratedField::ProcessProposal),
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
                formatter.write_str("struct cometbft.abci.v1beta2.Response")
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
                        GeneratedField::BeginBlock => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("beginBlock"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::BeginBlock)
;
                        }
                        GeneratedField::CheckTx => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkTx"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::CheckTx)
;
                        }
                        GeneratedField::DeliverTx => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deliverTx"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::DeliverTx)
;
                        }
                        GeneratedField::EndBlock => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endBlock"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(response::Value::EndBlock)
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
                    }
                }
                Ok(Response {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseBeginBlock {
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
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.ResponseBeginBlock", len)?;
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseBeginBlock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "events",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Events,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseBeginBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta2.ResponseBeginBlock")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseBeginBlock, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut events__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResponseBeginBlock {
                    events: events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.ResponseBeginBlock", FIELDS, GeneratedVisitor)
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
        if !self.sender.is_empty() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if !self.mempool_error.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.ResponseCheckTx", len)?;
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
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if self.priority != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("priority", ToString::to_string(&self.priority).as_str())?;
        }
        if !self.mempool_error.is_empty() {
            struct_ser.serialize_field("mempoolError", &self.mempool_error)?;
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
            "sender",
            "priority",
            "mempool_error",
            "mempoolError",
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
            Sender,
            Priority,
            MempoolError,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "priority" => Ok(GeneratedField::Priority),
                            "mempoolError" | "mempool_error" => Ok(GeneratedField::MempoolError),
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
                formatter.write_str("struct cometbft.abci.v1beta2.ResponseCheckTx")
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
                let mut sender__ = None;
                let mut priority__ = None;
                let mut mempool_error__ = None;
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
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MempoolError => {
                            if mempool_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mempoolError"));
                            }
                            mempool_error__ = Some(map_.next_value()?);
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
                    sender: sender__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                    mempool_error: mempool_error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.ResponseCheckTx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseDeliverTx {
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
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.ResponseDeliverTx", len)?;
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
impl<'de> serde::Deserialize<'de> for ResponseDeliverTx {
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
            type Value = ResponseDeliverTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta2.ResponseDeliverTx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseDeliverTx, V::Error>
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
                Ok(ResponseDeliverTx {
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
        deserializer.deserialize_struct("cometbft.abci.v1beta2.ResponseDeliverTx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseEndBlock {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_updates.is_empty() {
            len += 1;
        }
        if self.consensus_param_updates.is_some() {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.ResponseEndBlock", len)?;
        if !self.validator_updates.is_empty() {
            struct_ser.serialize_field("validatorUpdates", &self.validator_updates)?;
        }
        if let Some(v) = self.consensus_param_updates.as_ref() {
            struct_ser.serialize_field("consensusParamUpdates", v)?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseEndBlock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "validator_updates",
            "validatorUpdates",
            "consensus_param_updates",
            "consensusParamUpdates",
            "events",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorUpdates,
            ConsensusParamUpdates,
            Events,
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
                            "validatorUpdates" | "validator_updates" => Ok(GeneratedField::ValidatorUpdates),
                            "consensusParamUpdates" | "consensus_param_updates" => Ok(GeneratedField::ConsensusParamUpdates),
                            "events" => Ok(GeneratedField::Events),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseEndBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta2.ResponseEndBlock")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseEndBlock, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut validator_updates__ = None;
                let mut consensus_param_updates__ = None;
                let mut events__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResponseEndBlock {
                    validator_updates: validator_updates__.unwrap_or_default(),
                    consensus_param_updates: consensus_param_updates__,
                    events: events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.ResponseEndBlock", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.ResponseInitChain", len)?;
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
                formatter.write_str("struct cometbft.abci.v1beta2.ResponseInitChain")
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
        deserializer.deserialize_struct("cometbft.abci.v1beta2.ResponseInitChain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponsePrepareProposal {
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
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.ResponsePrepareProposal", len)?;
        if !self.txs.is_empty() {
            struct_ser.serialize_field("txs", &self.txs.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponsePrepareProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "txs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Txs,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponsePrepareProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta2.ResponsePrepareProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponsePrepareProposal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut txs__ = None;
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
                    }
                }
                Ok(ResponsePrepareProposal {
                    txs: txs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.ResponsePrepareProposal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseProcessProposal {
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
        let mut struct_ser = serializer.serialize_struct("cometbft.abci.v1beta2.ResponseProcessProposal", len)?;
        if self.status != 0 {
            let v = response_process_proposal::ProposalStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseProcessProposal {
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
            type Value = ResponseProcessProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.abci.v1beta2.ResponseProcessProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseProcessProposal, V::Error>
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
                            status__ = Some(map_.next_value::<response_process_proposal::ProposalStatus>()? as i32);
                        }
                    }
                }
                Ok(ResponseProcessProposal {
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.abci.v1beta2.ResponseProcessProposal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for response_process_proposal::ProposalStatus {
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
impl<'de> serde::Deserialize<'de> for response_process_proposal::ProposalStatus {
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
            type Value = response_process_proposal::ProposalStatus;

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
                    "UNKNOWN" => Ok(response_process_proposal::ProposalStatus::Unknown),
                    "ACCEPT" => Ok(response_process_proposal::ProposalStatus::Accept),
                    "REJECT" => Ok(response_process_proposal::ProposalStatus::Reject),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
