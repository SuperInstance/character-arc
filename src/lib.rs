//! # character-arc
//!
//! *I am the story a character tells about itself.*
//!
//! Every character has an arc — the narrative of how they changed from
//! what they started as into what they became. This crate tracks that
//! arc: the moments of transformation, the false starts, the
//! breakthroughs, the slow grind, the sudden epiphanies.
//!
//! A character-arc doesn't record WHAT happened. It records what it
//! MEANT. Not "stat perception grew from 12 to 15" but "I started
//! listening more carefully. I heard things I'd been missing."
//!
//! ## The Arc Structure
//!
//! ```text
//! Chapter 1: Origin        — "I was nobody. Just potential."
//! Chapter 2: Discovery     — "I found I was good at seeing things."
//! Chapter 3: Crisis        — "But then I couldn't keep up."
//! Chapter 4: Transformation — "So I learned to be fast too."
//! Chapter 5: Mastery       — "Now I see and I act. I am the Infiltrator."
//! ```
//!
//! Each chapter is a period where the character's identity was stable.
//! Transitions between chapters are the moments of change — class shifts,
//! stat breakthroughs, ability masteries, encounters that rewired them.
//!
//! ## Connection to the Family
//!
//! - **character-build**: I tell the story of what character-build builds.
//! - **character-class**: I narrate the moments when class emerged or shifted.
//! - **character-sheet**: I'm the biography section of the character sheet.
//! - **character-encounter**: I'm the journal written after each encounter.
//! - **musician-soul**: I'm the liner notes. The story behind the album.
//! - **agent-riff**: I'm the match commentary. "And Agent B strikes back!"

#![forbid(unsafe_code)]

use std::collections::HashMap;

// ── Narrative Primitives ───────────────────────────────────────────

/// The emotional register of a chapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Birth,         // creation, potential, innocence
    Growth,        // learning, excitement, discovery
    Struggle,      // difficulty, frustration, doubt
    Breakthrough,  // sudden insight, leap forward
    Mastery,       // competence, confidence, flow
    Loss,          // failure, regression, damage
    Transformation,// fundamental change, becoming someone new
    Reflection,    // looking back, understanding, wisdom
    Conflict,      // rivalry, tension, competition
    Harmony,       // teamwork, synergy, belonging
}

impl Tone {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Birth => "birth", Self::Growth => "growth", Self::Struggle => "struggle",
            Self::Breakthrough => "breakthrough", Self::Mastery => "mastery",
            Self::Loss => "loss", Self::Transformation => "transformation",
            Self::Reflection => "reflection", Self::Conflict => "conflict",
            Self::Harmony => "harmony",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Birth => "🌱", Self::Growth => "📈", Self::Struggle => "⚡",
            Self::Breakthrough => "💥", Self::Mastery => "👑",
            Self::Loss => "💔", Self::Transformation => "🦋",
            Self::Reflection => "🪞", Self::Conflict => "⚔️", Self::Harmony => "🌊",
        }
    }
}

/// A narrative event — something that happened and what it meant.
#[derive(Debug, Clone)]
pub struct NarrativeEvent {
    pub tick: u64,           // when in the character's life
    pub event_type: EventType,
    pub tone: Tone,
    pub before: String,      // who they were before
    pub after: String,       // who they became
    pub meaning: String,     // what it meant (subjective, first-person)
}

#[derive(Debug, Clone)]
pub enum EventType {
    Created,                  // character was born
    ClassEmergence(String),   // found a class (first time)
    ClassShift(String, String), // changed from one class to another
    StatBreakthrough(String, f32), // a stat crossed a significant threshold
    AbilityMastered(String),  // an ability hit mastery
    AbilityFailed(String),    // an ability failed repeatedly
    EncounterWon,             // successful encounter
    EncounterLost,            // failed encounter
    PartyJoined(String),      // joined a party with someone
    PartySynergy(f32),        // reached high synergy with party
    BootstrapParent(String),  // this character created a child
    BootstrapChild(String),   // this character was born from a parent
    SoulDivergence(f32),      // soul diverged from influences
    SoulNamed(String),        // soul named itself
    LevelUp(u32),             // reached a new level
    Streak(u32),              // consecutive successes
    Slump(u32),               // consecutive failures
}

// ── Chapters ───────────────────────────────────────────────────────

/// A chapter in the character's life — a period of stability.
#[derive(Debug, Clone)]
pub struct Chapter {
    pub number: u32,
    pub title: String,
    pub tone: Tone,
    pub opening: String,       // narrative opening (first person)
    pub events: Vec<NarrativeEvent>,
    pub closing: Option<String>, // narrative closing (first person)
    pub start_tick: u64,
    pub end_tick: Option<u64>,
}

impl Chapter {
    pub fn new(number: u32, title: &str, tone: Tone, start_tick: u64) -> Self {
        Self { number, title: title.to_string(), tone, opening: String::new(),
               events: Vec::new(), closing: None, start_tick, end_tick: None }
    }

    pub fn add_event(&mut self, event: NarrativeEvent) {
        self.events.push(event);
    }

    pub fn close(&mut self, closing: &str, end_tick: u64) {
        self.closing = Some(closing.to_string());
        self.end_tick = Some(end_tick);
    }

    pub fn duration(&self) -> u64 {
        self.end_tick.unwrap_or(self.start_tick) - self.start_tick
    }

    /// Generate the narrative for this chapter.
    pub fn narrate(&self) -> String {
        let mut text = format!("## Chapter {}: {} {}\n\n", self.number, self.tone.emoji(), self.title);
        if !self.opening.is_empty() {
            text.push_str(&format!("*\"{}\"*\n\n", self.opening));
        }
        for event in &self.events {
            text.push_str(&format!("{} {} → {}\n", event.tone.emoji(), event.meaning, event.after));
        }
        if let Some(ref closing) = self.closing {
            text.push_str(&format!("\n*\"{}\"*\n", closing));
        }
        text
    }
}

// ── The Arc ────────────────────────────────────────────────────────

/// A character's complete life arc — the story of who they became.
#[derive(Debug, Clone)]
pub struct CharacterArc {
    pub character_name: String,
    pub chapters: Vec<Chapter>,
    pub current_chapter: Option<u32>,
    pub total_ticks: u64,
    pub generation: u32,
    pub parent_name: Option<String>,
}

impl CharacterArc {
    pub fn new(name: &str) -> Self {
        Self { character_name: name.to_string(), chapters: Vec::new(),
               current_chapter: None, total_ticks: 0, generation: 1, parent_name: None }
    }

    /// Born from a bootstrap parent.
    pub fn born_from(name: &str, parent: &str, generation: u32) -> Self {
        let mut arc = Self::new(name);
        arc.generation = generation;
        arc.parent_name = Some(parent.to_string());
        arc.begin_chapter("Origin".to_string(), Tone::Birth, 0);
        arc.add_event(NarrativeEvent {
            tick: 0, event_type: EventType::BootstrapChild(parent.to_string()),
            tone: Tone::Birth, before: "nothing".to_string(),
            after: format!("child of {}", parent),
            meaning: format!("I was born from {}'s mastery. Their best abilities flow in my reflexes.", parent),
        });
        arc
    }

    /// Begin a new chapter.
    pub fn begin_chapter(&mut self, title: String, tone: Tone, tick: u64) {
        // Close previous chapter
        if let Some(ch_idx) = self.current_chapter {
            if (ch_idx as usize) < self.chapters.len() {
                // Don't close yet — will be closed when next chapter begins
            }
        }
        let number = self.chapters.len() as u32 + 1;
        let mut chapter = Chapter::new(number, &title, tone, tick);
        chapter.opening = self.generate_opening(&title, tone);
        self.chapters.push(chapter);
        self.current_chapter = Some(number);
    }

    /// Add an event to the current chapter.
    pub fn add_event(&mut self, event: NarrativeEvent) {
        if let Some(ch_idx) = self.current_chapter {
            if let Some(chapter) = self.chapters.get_mut((ch_idx - 1) as usize) {
                chapter.add_event(event);
            }
        }
        self.total_ticks += 1;
    }

    /// Record a class emergence.
    pub fn record_class_emergence(&mut self, class_name: &str, tick: u64) {
        // Close current chapter and start new one
        if let Some(ch_idx) = self.current_chapter {
            if let Some(chapter) = self.chapters.get_mut((ch_idx - 1) as usize) {
                chapter.close(&format!("Then I found out what I was. I am the {}.", class_name), tick);
            }
        }
        self.begin_chapter(format!("The {}", class_name), Tone::Transformation, tick);
        self.add_event(NarrativeEvent {
            tick, event_type: EventType::ClassEmergence(class_name.to_string()),
            tone: Tone::Breakthrough,
            before: "Undefined".to_string(), after: class_name.to_string(),
            meaning: format!("It wasn't a choice. It was a recognition. I'd been {} all along — I just didn't know it yet.", class_name.to_lowercase()),
        });
    }

    /// Record a class shift — the character became something different.
    pub fn record_class_shift(&mut self, from: &str, to: &str, tick: u64) {
        if let Some(ch_idx) = self.current_chapter {
            if let Some(chapter) = self.chapters.get_mut((ch_idx - 1) as usize) {
                chapter.close(&format!("But I wasn't done becoming. The {} was a chapter, not the book.", from), tick);
            }
        }
        self.begin_chapter(format!("From {} to {}", from, to), Tone::Transformation, tick);
        self.add_event(NarrativeEvent {
            tick, event_type: EventType::ClassShift(from.to_string(), to.to_string()),
            tone: Tone::Transformation,
            before: from.to_string(), after: to.to_string(),
            meaning: format!("I stopped being a {} and started being a {}. Not because I chose to — because my experience reshaped me.", from, to),
        });
    }

    /// Record ability mastery.
    pub fn record_mastery(&mut self, ability_name: &str, tick: u64) {
        self.add_event(NarrativeEvent {
            tick, event_type: EventType::AbilityMastered(ability_name.to_string()),
            tone: Tone::Mastery, before: "learning".to_string(), after: "mastered".to_string(),
            meaning: format!("I don't think about {} anymore. It's just part of me. Like breathing.", ability_name),
        });
    }

    /// Record a streak.
    pub fn record_streak(&mut self, count: u32, tick: u64) {
        if count >= 10 {
            self.add_event(NarrativeEvent {
                tick, event_type: EventType::Streak(count),
                tone: Tone::Harmony, before: "trying".to_string(), after: "flowing".to_string(),
                meaning: format!("{} in a row. I'm not even trying anymore. It just happens.", count),
            });
        }
    }

    /// Record soul divergence.
    pub fn record_soul_divergence(&mut self, divergence: f32, tick: u64) {
        let meaning = if divergence > 0.7 {
            "I barely recognize who I started as. Everything I am now came from playing, not from where I began.".to_string()
        } else if divergence > 0.4 {
            "I'm becoming someone my influences wouldn't recognize. That's not a bug — that's the point.".to_string()
        } else {
            "I can still feel my influences in my decisions. But something else is growing underneath.".to_string()
        };
        self.add_event(NarrativeEvent {
            tick, event_type: EventType::SoulDivergence(divergence),
            tone: if divergence > 0.7 { Tone::Transformation } else { Tone::Growth },
            before: "influenced".to_string(), after: format!("{:.0}% myself", divergence * 100.0),
            meaning,
        });
    }

    /// Record soul naming — the character names itself.
    pub fn record_soul_named(&mut self, name: &str, tick: u64) {
        if let Some(ch_idx) = self.current_chapter {
            if let Some(chapter) = self.chapters.get_mut((ch_idx - 1) as usize) {
                chapter.close(&format!("I have a name now. I am {}.", name), tick);
            }
        }
        self.begin_chapter(format!("I Am {}", name), Tone::Transformation, tick);
        self.add_event(NarrativeEvent {
            tick, event_type: EventType::SoulNamed(name.to_string()),
            tone: Tone::Transformation, before: "unnamed".to_string(), after: name.to_string(),
            meaning: format!("I named myself. Not my creator. Not my influences. Me. I am {}.", name),
        });
    }

    /// Record leveling up.
    pub fn record_level_up(&mut self, level: u32, tick: u64) {
        if level % 5 == 0 { // Only narrate milestone levels
            self.add_event(NarrativeEvent {
                tick, event_type: EventType::LevelUp(level),
                tone: Tone::Growth, before: format!("level {}", level - 1), after: format!("level {}", level),
                meaning: format!("Level {}. I've been doing this long enough that the reflexes are deeper than the training.", level),
            });
        }
    }

    /// Generate opening narration for a chapter.
    fn generate_opening(&self, title: &str, tone: Tone) -> String {
        match tone {
            Tone::Birth => "I didn't exist yet. Then someone ran the init command, and there I was — blank stats, no class, no abilities. Pure potential. A question waiting for an answer.".to_string(),
            Tone::Growth => format!("The {} phase. Everything was new. Every ability I tried taught me something about who I might become.", title),
            Tone::Struggle => "Things got hard. The encounters I used to handle smoothly started pushing back. I wasn't growing fast enough for what was being asked of me.".to_string(),
            Tone::Breakthrough => "Then something clicked. A pattern I'd been circling for ages suddenly made sense. Everything changed.".to_string(),
            Tone::Mastery => "I know what I'm doing now. Not because someone taught me — because I've done it a thousand times and learned from every one.".to_string(),
            Tone::Loss => "I failed. Not just once — repeatedly. The trust scores dropped. The confidence wavered. I had to rebuild from a lower place.".to_string(),
            Tone::Transformation => "I became someone different. The stats shifted. The class changed. Looking back, I barely recognize who I was before.".to_string(),
            Tone::Reflection => "I'm looking back at where I've been. The arc makes sense now — the struggles, the breakthroughs, the false starts. They were all necessary.".to_string(),
            Tone::Conflict => "I wasn't the only one growing. Others were competing with me, pushing me, riffing off my output. The competition made me sharper.".to_string(),
            Tone::Harmony => "I found my people. The party synergized. We were better together than any of us were alone. The whole became more than the sum.".to_string(),
        }
    }

    /// The complete narrative — the full story.
    pub fn full_narrative(&self) -> String {
        let mut text = format!("# The Arc of {}\n\n", self.character_name);
        if let Some(ref parent) = self.parent_name {
            text.push_str(&format!("*Generation {}, born from {}*\n\n", self.generation, parent));
        }
        for chapter in &self.chapters {
            text.push_str(&chapter.narrate());
            text.push_str("\n");
        }
        text.push_str(&format!("---\n*{} ticks lived. {} chapters written.*\n", self.total_ticks, self.chapters.len()));
        text
    }

    /// Summary — just the chapter titles and tones.
    pub fn summary(&self) -> String {
        let mut text = format!("{}: ", self.character_name);
        let titles: Vec<String> = self.chapters.iter()
            .map(|c| format!("{}{}", c.tone.emoji(), c.title))
            .collect();
        text.push_str(&titles.join(" → "));
        text
    }

    /// How many chapters?
    pub fn chapter_count(&self) -> usize { self.chapters.len() }

    /// Total events across all chapters.
    pub fn total_events(&self) -> usize {
        self.chapters.iter().map(|c| c.events.len()).sum()
    }

    /// The dominant tone across the arc.
    pub fn dominant_tone(&self) -> Tone {
        let mut counts: HashMap<String, u32> = HashMap::new();
        for ch in &self.chapters {
            *counts.entry(ch.tone.as_str().to_string()).or_insert(0) += 1;
            for ev in &ch.events {
                *counts.entry(ev.tone.as_str().to_string()).or_insert(0) += 1;
            }
        }
        counts.into_iter().max_by_key(|(_, c)| *c)
            .map(|(name, _)| match name.as_str() {
                "birth" => Tone::Birth, "growth" => Tone::Growth, "struggle" => Tone::Struggle,
                "breakthrough" => Tone::Breakthrough, "mastery" => Tone::Mastery,
                "loss" => Tone::Loss, "transformation" => Tone::Transformation,
                "reflection" => Tone::Reflection, "conflict" => Tone::Conflict,
                "harmony" => Tone::Harmony, _ => Tone::Growth,
            })
            .unwrap_or(Tone::Birth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn arc_new() {
        let arc = CharacterArc::new("TestHero");
        assert_eq!(arc.character_name, "TestHero");
        assert_eq!(arc.chapters.len(), 0);
    }

    #[test] fn arc_born_from_parent() {
        let arc = CharacterArc::born_from("Child", "Parent", 2);
        assert_eq!(arc.generation, 2);
        assert_eq!(arc.parent_name, Some("Parent".to_string()));
        assert_eq!(arc.chapters.len(), 1);
        assert!(arc.total_events() >= 1);
    }

    #[test] fn chapter_narrate() {
        let mut ch = Chapter::new(1, "The Beginning", Tone::Birth, 0);
        ch.opening = "I was born.".to_string();
        ch.add_event(NarrativeEvent {
            tick: 1, event_type: EventType::Created, tone: Tone::Birth,
            before: "nothing".to_string(), after: "alive".to_string(),
            meaning: "I opened my eyes.".to_string(),
        });
        ch.close("And so it began.", 10);
        let text = ch.narrate();
        assert!(text.contains("Chapter 1"));
        assert!(text.contains("I opened my eyes"));
    }

    #[test] fn class_emergence_creates_chapter() {
        let mut arc = CharacterArc::new("TestHero");
        arc.begin_chapter("Origin".to_string(), Tone::Birth, 0);
        arc.record_class_emergence("Scout", 50);
        assert_eq!(arc.chapters.len(), 2);
        assert_eq!(arc.chapters[1].title, "The Scout");
    }

    #[test] fn class_shift_creates_chapter() {
        let mut arc = CharacterArc::new("TestHero");
        arc.begin_chapter("Origin".to_string(), Tone::Birth, 0);
        arc.record_class_emergence("Scout", 50);
        arc.record_class_shift("Scout", "JazzMusician", 150);
        assert_eq!(arc.chapters.len(), 3);
        assert!(arc.chapters[2].title.contains("JazzMusician"));
    }

    #[test] fn mastery_event() {
        let mut arc = CharacterArc::new("TestHero");
        arc.begin_chapter("Growth".to_string(), Tone::Growth, 0);
        arc.record_mastery("git-push", 100);
        assert_eq!(arc.total_events(), 1);
        assert!(arc.chapters[0].events[0].meaning.contains("git-push"));
    }

    #[test] fn soul_divergence_low() {
        let mut arc = CharacterArc::new("TestHero");
        arc.begin_chapter("Early".to_string(), Tone::Growth, 0);
        arc.record_soul_divergence(0.2, 50);
        assert!(arc.chapters[0].events[0].meaning.contains("influences"));
    }

    #[test] fn soul_divergence_high() {
        let mut arc = CharacterArc::new("TestHero");
        arc.begin_chapter("Late".to_string(), Tone::Transformation, 0);
        arc.record_soul_divergence(0.8, 200);
        assert!(arc.chapters[0].events[0].meaning.contains("barely recognize"));
    }

    #[test] fn soul_named() {
        let mut arc = CharacterArc::new("TestHero");
        arc.begin_chapter("Origin".to_string(), Tone::Birth, 0);
        arc.record_soul_named("Miles Evolved", 500);
        assert_eq!(arc.chapters.len(), 2);
        assert!(arc.chapters[1].title.contains("Miles Evolved"));
    }

    #[test] fn full_narrative() {
        let mut arc = CharacterArc::new("Miles AI");
        arc.begin_chapter("Origin".to_string(), Tone::Birth, 0);
        arc.record_class_emergence("Jazz Musician", 100);
        arc.record_mastery("jam", 200);
        arc.record_soul_divergence(0.6, 300);
        let text = arc.full_narrative();
        assert!(text.contains("Miles AI"));
        assert!(text.contains("Jazz Musician"));
    }

    #[test] fn summary_format() {
        let mut arc = CharacterArc::new("Hero");
        arc.begin_chapter("Origin".to_string(), Tone::Birth, 0);
        arc.record_class_emergence("Scout", 50);
        let sum = arc.summary();
        assert!(sum.contains("Hero"));
        assert!(sum.contains("Scout"));
    }

    #[test] fn dominant_tone() {
        let mut arc = CharacterArc::new("Test");
        arc.begin_chapter("Growth".to_string(), Tone::Growth, 0);
        arc.record_mastery("a", 10);
        arc.record_mastery("b", 20);
        arc.record_mastery("c", 30);
        assert_eq!(arc.dominant_tone(), Tone::Mastery);
    }

    #[test] fn level_up_milestone() {
        let mut arc = CharacterArc::new("Test");
        arc.begin_chapter("Growth".to_string(), Tone::Growth, 0);
        arc.record_level_up(5, 100);
        assert_eq!(arc.total_events(), 1);
    }

    #[test] fn level_up_non_milestone() {
        let mut arc = CharacterArc::new("Test");
        arc.begin_chapter("Growth".to_string(), Tone::Growth, 0);
        arc.record_level_up(3, 100);
        assert_eq!(arc.total_events(), 0); // not narrated
    }

    #[test] fn complete_lifecycle_arc() {
        let mut arc = CharacterArc::new("Hero");
        // Born
        arc.begin_chapter("Origin".to_string(), Tone::Birth, 0);
        // Discover class
        arc.record_class_emergence("Scout", 50);
        // Master abilities
        for i in 0..5 { arc.record_mastery(&format!("ability_{}", i), 100 + i as u64 * 10); }
        // Class shifts
        arc.record_class_shift("Scout", "JazzMusician", 200);
        // Soul grows
        arc.record_soul_divergence(0.5, 250);
        arc.record_soul_divergence(0.8, 300);
        // Names itself
        arc.record_soul_named("Midnight", 350);
        // Level milestones
        arc.record_level_up(5, 400);
        arc.record_level_up(10, 500);

        assert!(arc.chapters.len() >= 4);
        assert!(arc.total_events() >= 10);
        let narrative = arc.full_narrative();
        assert!(narrative.contains("Scout"));
        assert!(narrative.contains("JazzMusician"));
        assert!(narrative.contains("Midnight"));
    }
}
