pub use super::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MimeType {
    Png,
    Jpeg,
    Tiff,
    Bmp,
    Gif,
}

impl TryFrom<&str> for MimeType {
    type Error = crate::Error;
    fn try_from(inp: &str) -> crate::Result<Self> {
        Ok(match inp {
            "image/jpeg" => MimeType::Jpeg,
            "image/png" => MimeType::Png,
            "image/tiff" => MimeType::Tiff,
            "image/bmp" => MimeType::Bmp,
            "image/gif" => MimeType::Gif,
            _ => return Err(crate::Error::UnsupportedMimeType(inp.to_owned())),
        })
    }
}

impl From<MimeType> for &'static str {
    fn from(mt: MimeType) -> Self {
        match mt {
            MimeType::Jpeg => "image/jpeg",
            MimeType::Png => "image/png",
            MimeType::Tiff => "image/tiff",
            MimeType::Bmp => "image/bmp",
            MimeType::Gif => "image/gif",
        }
    }
}

impl From<MimeType> for String {
    fn from(mt: MimeType) -> Self {
        <MimeType as Into<&'static str>>::into(mt).to_owned()
    }
}

impl From<&mp4ameta::ImgFmt> for MimeType {
    fn from(fmt: &mp4ameta::ImgFmt) -> Self {
        match fmt {
            mp4ameta::ImgFmt::Png => Self::Png,
            mp4ameta::ImgFmt::Bmp => Self::Bmp,
            mp4ameta::ImgFmt::Jpeg => Self::Jpeg,
        }
    }
}

impl TryFrom<&MimeType> for mp4ameta::ImgFmt {
    type Error = crate::Error;
    fn try_from(mime: &MimeType) -> crate::Result<Self> {
        Ok(match mime {
            MimeType::Jpeg => Self::Jpeg,
            MimeType::Png => Self::Png,
            MimeType::Bmp => Self::Bmp,

            _ => return Err(crate::Error::UnsupportedMimeType(format!("{:?}", mime))),
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Picture<'a> {
    pub data: &'a [u8],
    pub mime_type: MimeType,
}

impl<'a> Picture<'a> {
    pub fn new(data: &'a [u8], mime_type: MimeType) -> Self {
        Self { data, mime_type }
    }
}

impl<'a> From<Picture<'_>> for id3::frame::Picture {
    fn from(p: Picture<'_>) -> Self {
        Self {
            mime_type: p.mime_type.into(),
            picture_type: id3::frame::PictureType::CoverFront,
            description: String::new(),
            data: p.data.to_vec(),
        }
    }
}

impl<'a> From<&Picture<'_>> for id3::frame::Picture {
    fn from(p: &Picture<'_>) -> Self {
        Self {
            mime_type: p.mime_type.into(),
            picture_type: id3::frame::PictureType::CoverFront,
            description: String::new(),
            data: p.data.to_vec(),
        }
    }
}

impl<'a> TryFrom<Picture<'a>> for mp4ameta::Img<&'a [u8]> {
    type Error = crate::Error;

    fn try_from(p: Picture<'a>) -> Result<Self> {
        Ok(Self {
            fmt: (&p.mime_type).try_into()?,
            data: p.data,
        })
    }
}

impl<'a> TryFrom<&Picture<'a>> for mp4ameta::Img<&'a [u8]> {
    type Error = crate::Error;

    fn try_from(p: &Picture<'a>) -> Result<Self> {
        Ok(Self {
            fmt: (&p.mime_type).try_into()?,
            data: p.data,
        })
    }
}

/// A struct for representing an album for convenience.
#[derive(Debug)]
pub struct Album<'a> {
    pub title: &'a str,
    pub artist: Option<&'a str>,
    pub cover: Option<Picture<'a>>,
}

impl<'a> Album<'a> {
    pub fn with_title(title: &'a str) -> Self {
        Self {
            title,
            artist: None,
            cover: None,
        }
    }
    pub fn and_artist(mut self, artist: &'a str) -> Self {
        self.artist = Some(artist);
        self
    }
    pub fn and_cover(mut self, cover: Picture<'a>) -> Self {
        self.cover = Some(cover);
        self
    }
    pub fn with_all(title: &'a str, artist: &'a str, cover: Picture<'a>) -> Self {
        Self {
            title,
            artist: Some(artist),
            cover: Some(cover),
        }
    }
}

// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
// pub enum PictureType {
//     Other,
//     Icon,
//     OtherIcon,
//     CoverFront,
//     CoverBack,
//     Leaflet,
//     Media,
//     LeadArtist,
//     Artist,
//     Conductor,
//     Band,
//     Composer,
//     Lyricist,
//     RecordingLocation,
//     DuringRecording,
//     DuringPerformance,
//     ScreenCapture,
//     BrightFish,
//     Illustration,
//     BandLogo,
//     PublisherLogo,
//     Undefined(u8),
// }
