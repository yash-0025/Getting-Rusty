# Dual English + Hinglish Explanations & Reference Documentation

## Context & Purpose
Technical systems programming concepts in Rust (ownership, borrow checker, lifetimes, fearless concurrency, smart pointers, async runtimes, compile-time SQL verification) can easily become dry, tedious, and cognitively exhausting when presented purely in academic jargon. To make learning engaging, intuitive, and fun, the learner prefers explanations in everyday **Hinglish** (Hindi written in Roman/English script, combined naturally with technical English terms).

## Core Directives

1. **Dual Explanation in Chat Responses**:
   - Whenever explaining concepts, system architecture, data flows, code walkthroughs, design trade-offs, exercises, or Rust decisions, ALWAYS provide BOTH:
     - Clear, professional technical English (including domain ELI5 analogies per Rule 8).
     - An engaging, punchy, fun, and crystal-clear **Hinglish technical breakdown** explaining what is actually happening "under the hood" from an engineering perspective.

2. **Knowledgeable + Fun to Read Blend in `hinglish-docs.md`**:
   - Do NOT write dry, academic translations of technical English, and do NOT isolate stories from technical facts.
   - Deliver a seamless blend of **deep technical knowledge + engaging, fun, conversational delivery**:
     - Explain real engineering problems, memory models (stack vs heap), borrow checker rules, zero-cost abstractions, monomorphization, async runtimes (Tokio task scheduling), and Rust typing decisions.
     - Frame them with funny, memorable, relatable intuition (e.g. "Stack sticky note hai, Heap library book hai", "Ownership matlab single library book rule", "Arc taxi company ka central radio system hai", "sqlx brick factory ka inspector hai jo compile time pe hi pakad leta hai").
   - Every Hinglish blended explanation provided must be stored verbatim in `hinglish-docs.md` (no omitting, no paraphrasing, per Rule 16).

3. **Retroactive Coverage**:
   - Maintain a running chronicle in `hinglish-docs.md` starting from Day 1 through all future days without gaps.