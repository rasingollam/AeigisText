use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Keyword {
    pub word: String,
    pub count: usize,
}

#[derive(Serialize)]
pub struct AnalysisResult {
    pub word_count: usize,
    pub sentence_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<Keyword>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

pub struct AnalyseOptions {
    pub keywords: bool,
    pub sentiment: bool,
    pub summary: bool,
}

fn default_stopwords() -> Vec<&'static str> {
    vec![
        "the", "and", "a", "an", "in", "on", "of", "to", "is", "it", "that", "this",
        "for", "with", "as", "are", "was", "were", "by", "be", "or", "from", "at",
    ]
}

fn sentiment_lexicons() -> (Vec<&'static str>, Vec<&'static str>) {
    let positive = vec!["good", "great", "excellent", "positive", "happy", "love", "like", "awesome", "nice", "best"];
    let negative = vec!["bad", "terrible", "poor", "negative", "sad", "hate", "dislike", "awful", "worst"];
    (positive, negative)
}

pub fn analyze_text(text: &str, opts: &AnalyseOptions) -> AnalysisResult {
    let word_re = Regex::new(r"\p{L}+").unwrap();
    let sentence_re = Regex::new(r"[^.!?]+[.!?]?").unwrap();

    let mut words: Vec<String> = Vec::new();
    for cap in word_re.find_iter(text) {
        words.push(cap.as_str().to_lowercase());
    }

    let word_count = words.len();

    let sentences: Vec<&str> = if text.trim().is_empty() {
        vec![]
    } else {
        sentence_re.find_iter(text.trim()).map(|m| m.as_str()).collect()
    };
    let sentence_count = sentences.iter().filter(|s| !s.trim().is_empty()).count();

    let mut keywords: Option<Vec<Keyword>> = None;
    if opts.keywords {
        let stopwords: Vec<&str> = default_stopwords();
        let mut freq: HashMap<String, usize> = HashMap::new();
        for w in &words {
            if stopwords.contains(&w.as_str()) {
                continue;
            }
            *freq.entry(w.clone()).or_insert(0) += 1;
        }
        let mut items: Vec<(String, usize)> = freq.into_iter().collect();
        items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        let top: Vec<Keyword> = items.into_iter().take(5).map(|(w, c)| Keyword { word: w, count: c }).collect();
        keywords = Some(top);
    }

    let mut sentiment: Option<String> = None;
    if opts.sentiment {
        let (pos_list, neg_list) = sentiment_lexicons();
        let mut pos = 0usize;
        let mut neg = 0usize;
        for w in &words {
            if pos_list.contains(&w.as_str()) {
                pos += 1;
            }
            if neg_list.contains(&w.as_str()) {
                neg += 1;
            }
        }
        sentiment = Some(if pos > neg {
            "positive".to_string()
        } else if neg > pos {
            "negative".to_string()
        } else {
            "neutral".to_string()
        });
    }

    let mut summary: Option<String> = None;
    if opts.summary {
        let mut first_sentences: Vec<&str> = sentences.into_iter().filter(|s| !s.trim().is_empty()).collect();
        if !first_sentences.is_empty() {
            let take = if first_sentences.len() >= 2 { 2 } else { 1 };
            let s: Vec<&str> = first_sentences.drain(0..take).collect();
            summary = Some(s.join(" ").trim().to_string());
        } else {
            summary = Some("".to_string());
        }
    }

    AnalysisResult {
        word_count,
        sentence_count,
        keywords,
        sentiment,
        summary,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_analysis() {
        let text = "This is a test. It is great! But some parts are bad.";
        let opts = AnalyseOptions { keywords: true, sentiment: true, summary: true };
        let res = analyze_text(text, &opts);
        assert_eq!(res.word_count, 12);
        assert_eq!(res.sentence_count, 3);
        assert!(res.keywords.is_some());
        assert!(res.sentiment.is_some());
        assert!(res.summary.is_some());
    }
}
