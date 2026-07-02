use anyhow::Result;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub index: usize,
    pub content: String,
    pub start_offset: usize,
    pub end_offset: usize,
    pub target_length: usize,
}

pub struct FileChunker {
    pub chunk_size: usize,
    pub overlap: usize,
}

impl FileChunker {
    pub fn new(chunk_size: usize, overlap: usize) -> Self {
        Self { chunk_size, overlap }
    }

    /// Read file and split into chunks
    pub fn chunk_file(&self, path: &Path) -> Result<Vec<Chunk>> {
        let content = std::fs::read_to_string(path)?;
        Ok(self.chunk_text(&content))
    }

    /// Split text into chunks respecting paragraph boundaries
    pub fn chunk_text(&self, text: &str) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let mut chunks = Vec::new();
        let mut start = 0;

        while start < text.len() {
            let ideal_end = (start + self.chunk_size).min(text.len());

            // Try to break at paragraph boundary
            let end = if ideal_end < text.len() {
                // Look for double newline (paragraph break) near the ideal end
                let search_start = (ideal_end as isize - (self.chunk_size as isize / 4)).max(0) as usize;
                let search_range = &text[search_start..ideal_end.min(text.len())];

                if let Some(pos) = search_range.rfind("\n\n") {
                    search_start + pos + 2 // Skip past the double newline
                } else if let Some(pos) = search_range.rfind('\n') {
                    search_start + pos + 1 // Skip past the newline
                } else {
                    ideal_end
                }
            } else {
                text.len()
            };

            let content = text[start..end].to_string();
            let target_length = self.chunk_size;

            chunks.push(Chunk {
                index: chunks.len(),
                content,
                start_offset: start,
                end_offset: end,
                target_length,
            });

            if end >= text.len() {
                break;
            }

            // Move start back by overlap, but don't go backwards
            let next_start = end.saturating_sub(self.overlap);
            if next_start <= start {
                start = end;
            } else {
                start = next_start;
            }

            // Safety: if we didn't advance, force advance
            if start >= text.len() {
                break;
            }
        }

        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_empty_text_returns_no_chunks() {
        let chunker = FileChunker::new(100, 20);
        let chunks = chunker.chunk_text("");
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_single_chunk_when_text_smaller_than_chunk_size() {
        let chunker = FileChunker::new(1000, 20);
        let text = "This is a short text.";
        let chunks = chunker.chunk_text(text);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].content, text);
        assert_eq!(chunks[0].index, 0);
        assert_eq!(chunks[0].start_offset, 0);
        assert_eq!(chunks[0].end_offset, text.len());
    }

    #[test]
    fn test_respects_paragraph_boundaries() {
        let chunker = FileChunker::new(50, 10);
        let text = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
        let chunks = chunker.chunk_text(text);

        assert!(chunks.len() >= 2);

        let first_chunk_content = &chunks[0].content;
        assert!(first_chunk_content.ends_with('\n') || first_chunk_content.contains("\n\n"));
    }

    #[test]
    fn test_overlap_between_chunks() {
        let chunker = FileChunker::new(100, 20);
        let text = "A".repeat(200);
        let chunks = chunker.chunk_text(&text);

        if chunks.len() >= 2 {
            let chunk1_end = chunks[0].end_offset;
            let chunk2_start = chunks[1].start_offset;
            let overlap = chunk1_end - chunk2_start;

            assert!(overlap >= 10, "Overlap should be at least 10, got {}", overlap);
            assert!(overlap <= 30, "Overlap should be at most 30, got {}", overlap);
        }
    }

    #[test]
    fn test_chunk_file_reads_actual_file() {
        let chunker = FileChunker::new(50, 10);
        let mut temp_file = NamedTempFile::new().unwrap();

        writeln!(temp_file, "Line one\n\nLine two\n\nLine three").unwrap();

        let chunks = chunker.chunk_file(temp_file.path()).unwrap();
        assert!(!chunks.is_empty());
        assert!(chunks[0].content.contains("Line one"));
    }

    #[test]
    fn test_multiple_chunks_created() {
        let chunker = FileChunker::new(100, 20);
        let text = "Paragraph one.\n\nParagraph two.\n\nParagraph three.\n\n".repeat(10);
        let chunks = chunker.chunk_text(&text);

        assert!(chunks.len() > 1, "Should create multiple chunks");
    }
}
