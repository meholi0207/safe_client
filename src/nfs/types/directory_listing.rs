// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.
use super::file::File;
use super::metadata::Metadata;
use super::directory_info::DirectoryInfo;
use routing;
use routing::test_utils::Random;
use std::fmt;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DirectoryListing {
    id: routing::NameType,
    metadata: Metadata,
    sub_directories: Vec<DirectoryInfo>,
    files: Vec<File>
}

impl DirectoryListing {
    pub fn new(name: String, user_metadata: Vec<u8>) -> DirectoryListing {
        DirectoryListing {
            id: routing::test_utils::Random::generate_random(),
            metadata: Metadata::new(name, user_metadata),
            sub_directories: Vec::new(),
            files: Vec::new()
        }
    }

    pub fn get_metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }

    pub fn get_files(&self) -> Vec<File> {
        self.files.clone()
    }

    pub fn set_files(&mut self, files: Vec<File>) {
        self.files = files;
    }

    pub fn get_sub_directories(&self) -> Vec<DirectoryInfo> {
        self.sub_directories.clone()
    }

    pub fn set_sub_directories(&mut self, dirs: Vec<DirectoryInfo>) {
        self.sub_directories = dirs;
    }

    pub fn set_name(&mut self, name: String) {
        self.metadata.set_name(name);
    }

    pub fn get_id(&self) -> routing::NameType {
        self.id.clone()
    }
}

impl fmt::Debug for DirectoryListing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {}, metadata: {}", self.id, self.metadata)
    }
}

impl fmt::Display for DirectoryListing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {}, metadata: {}", self.id, self.metadata)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::metadata::Metadata;
    use cbor;

    #[test]
    fn serialise() {
        let obj_before = DirectoryListing::new("Home".to_string(), "{mime:\"application/json\"}".to_string().into_bytes());

        let mut e = cbor::Encoder::from_memory();
        e.encode(&[&obj_before]).unwrap();

        let mut d = cbor::Decoder::from_bytes(e.as_bytes());
        let obj_after: DirectoryListing = d.decode().next().unwrap().unwrap();

        assert_eq!(obj_before, obj_after);
    }
}