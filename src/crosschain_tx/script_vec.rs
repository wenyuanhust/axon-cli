// Generated by Molecule 0.7.0
#![allow(unused_imports)]

use ckb_types::molecule;
use ckb_types::packed::*;
use ckb_types::prelude::*;
// these lines above are manually added
// replace "::molecule" to "molecule" in below code

#[derive(Clone)]
pub struct ScriptVec(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ScriptVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ScriptVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ScriptVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl ::core::default::Default for ScriptVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![4, 0, 0, 0];
        ScriptVec::new_unchecked(v.into())
    }
}
impl ScriptVec {
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }

    pub fn item_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }

    pub fn len(&self) -> usize {
        self.item_count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, idx: usize) -> Option<Script> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }

    pub fn get_unchecked(&self, idx: usize) -> Script {
        let slice = self.as_slice();
        let start_idx = molecule::NUMBER_SIZE * (1 + idx);
        let start = molecule::unpack_number(&slice[start_idx..]) as usize;
        if idx == self.len() - 1 {
            Script::new_unchecked(self.0.slice(start..))
        } else {
            let end_idx = start_idx + molecule::NUMBER_SIZE;
            let end = molecule::unpack_number(&slice[end_idx..]) as usize;
            Script::new_unchecked(self.0.slice(start..end))
        }
    }

    pub fn as_reader<'r>(&'r self) -> ScriptVecReader<'r> {
        ScriptVecReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ScriptVec {
    type Builder = ScriptVecBuilder;

    const NAME: &'static str = "ScriptVec";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ScriptVec(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ScriptVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ScriptVecReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
#[derive(Clone, Copy)]
pub struct ScriptVecReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ScriptVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ScriptVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ScriptVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ScriptVecReader<'r> {
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }

    pub fn item_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }

    pub fn len(&self) -> usize {
        self.item_count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, idx: usize) -> Option<ScriptReader<'r>> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }

    pub fn get_unchecked(&self, idx: usize) -> ScriptReader<'r> {
        let slice = self.as_slice();
        let start_idx = molecule::NUMBER_SIZE * (1 + idx);
        let start = molecule::unpack_number(&slice[start_idx..]) as usize;
        if idx == self.len() - 1 {
            ScriptReader::new_unchecked(&self.as_slice()[start..])
        } else {
            let end_idx = start_idx + molecule::NUMBER_SIZE;
            let end = molecule::unpack_number(&slice[end_idx..]) as usize;
            ScriptReader::new_unchecked(&self.as_slice()[start..end])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for ScriptVecReader<'r> {
    type Entity = ScriptVec;

    const NAME: &'static str = "ScriptVecReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        ScriptVecReader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(
                Self,
                TotalSizeNotMatch,
                molecule::NUMBER_SIZE * 2,
                slice_len
            );
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        for pair in offsets.windows(2) {
            let start = pair[0];
            let end = pair[1];
            ScriptReader::verify(&slice[start..end], compatible)?;
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct ScriptVecBuilder(pub(crate) Vec<Script>);
impl ScriptVecBuilder {
    pub fn set(mut self, v: Vec<Script>) -> Self {
        self.0 = v;
        self
    }

    pub fn push(mut self, v: Script) -> Self {
        self.0.push(v);
        self
    }

    pub fn extend<T: ::core::iter::IntoIterator<Item = Script>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
}
impl molecule::prelude::Builder for ScriptVecBuilder {
    type Entity = ScriptVec;

    const NAME: &'static str = "ScriptVecBuilder";

    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (self.0.len() + 1)
            + self
                .0
                .iter()
                .map(|inner| inner.as_slice().len())
                .sum::<usize>()
    }

    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let item_count = self.0.len();
        if item_count == 0 {
            writer.write_all(&molecule::pack_number(
                molecule::NUMBER_SIZE as molecule::Number,
            ))?;
        } else {
            let (total_size, offsets) = self.0.iter().fold(
                (
                    molecule::NUMBER_SIZE * (item_count + 1),
                    Vec::with_capacity(item_count),
                ),
                |(start, mut offsets), inner| {
                    offsets.push(start);
                    (start + inner.as_slice().len(), offsets)
                },
            );
            writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
            for offset in offsets.into_iter() {
                writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
            }
            for inner in self.0.iter() {
                writer.write_all(inner.as_slice())?;
            }
        }
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ScriptVec::new_unchecked(inner.into())
    }
}
pub struct ScriptVecIterator(ScriptVec, usize, usize);
impl ::core::iter::Iterator for ScriptVecIterator {
    type Item = Script;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::core::iter::ExactSizeIterator for ScriptVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::core::iter::IntoIterator for ScriptVec {
    type IntoIter = ScriptVecIterator;
    type Item = Script;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        ScriptVecIterator(self, 0, len)
    }
}
impl<'r> ScriptVecReader<'r> {
    pub fn iter<'t>(&'t self) -> ScriptVecReaderIterator<'t, 'r> {
        ScriptVecReaderIterator(&self, 0, self.len())
    }
}
pub struct ScriptVecReaderIterator<'t, 'r>(&'t ScriptVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::core::iter::Iterator for ScriptVecReaderIterator<'t, 'r> {
    type Item = ScriptReader<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::core::iter::ExactSizeIterator for ScriptVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct ScriptVecOpt(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ScriptVecOpt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ScriptVecOpt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ScriptVecOpt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(v) = self.to_opt() {
            write!(f, "{}(Some({}))", Self::NAME, v)
        } else {
            write!(f, "{}(None)", Self::NAME)
        }
    }
}
impl ::core::default::Default for ScriptVecOpt {
    fn default() -> Self {
        let v: Vec<u8> = vec![];
        ScriptVecOpt::new_unchecked(v.into())
    }
}
impl ScriptVecOpt {
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn to_opt(&self) -> Option<ScriptVec> {
        if self.is_none() {
            None
        } else {
            Some(ScriptVec::new_unchecked(self.0.clone()))
        }
    }

    pub fn as_reader<'r>(&'r self) -> ScriptVecOptReader<'r> {
        ScriptVecOptReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ScriptVecOpt {
    type Builder = ScriptVecOptBuilder;

    const NAME: &'static str = "ScriptVecOpt";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ScriptVecOpt(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ScriptVecOptReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ScriptVecOptReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_opt())
    }
}
#[derive(Clone, Copy)]
pub struct ScriptVecOptReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ScriptVecOptReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ScriptVecOptReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ScriptVecOptReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        if let Some(v) = self.to_opt() {
            write!(f, "{}(Some({}))", Self::NAME, v)
        } else {
            write!(f, "{}(None)", Self::NAME)
        }
    }
}
impl<'r> ScriptVecOptReader<'r> {
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_some(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn to_opt(&self) -> Option<ScriptVecReader<'r>> {
        if self.is_none() {
            None
        } else {
            Some(ScriptVecReader::new_unchecked(self.as_slice()))
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for ScriptVecOptReader<'r> {
    type Entity = ScriptVecOpt;

    const NAME: &'static str = "ScriptVecOptReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        ScriptVecOptReader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        if !slice.is_empty() {
            ScriptVecReader::verify(&slice[..], compatible)?;
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct ScriptVecOptBuilder(pub(crate) Option<ScriptVec>);
impl ScriptVecOptBuilder {
    pub fn set(mut self, v: Option<ScriptVec>) -> Self {
        self.0 = v;
        self
    }
}
impl molecule::prelude::Builder for ScriptVecOptBuilder {
    type Entity = ScriptVecOpt;

    const NAME: &'static str = "ScriptVecOptBuilder";

    fn expected_length(&self) -> usize {
        self.0
            .as_ref()
            .map(|ref inner| inner.as_slice().len())
            .unwrap_or(0)
    }

    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        self.0
            .as_ref()
            .map(|ref inner| writer.write_all(inner.as_slice()))
            .unwrap_or(Ok(()))
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ScriptVecOpt::new_unchecked(inner.into())
    }
}
