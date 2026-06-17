<!--
Source Session: ses_2110033e2ffe4Y0UsY5ppNdnvX
Part ID: prt_deeffcc22002DvfH19t8w0iVla
Character Count: 16183
Extracted: 2026-06-17T07:50:39Z
-->

TASK: Create a professional-grade PDF using weasyprint (HTML/CSS → PDF). This replaces a reportlab-based PDF that had design issues. The HTML/CSS approach gives us clickable links, real backgrounds, proper typography, and CSS-based diagrams.

EXPECTED OUTCOME: A single PDF at /home/jon/code/life-skills-advocates/life-skills-advocates-notes/summary-pdf/coaching-preparation-report.pdf

REQUIRED TOOLS: Write (create HTML file), Bash (run weasyprint to convert HTML to PDF)

MUST DO:

1. **Create an HTML file** at /home/jon/code/life-skills-advocates/life-skills-advocates-notes/summary-pdf/report.html that contains ALL content and styling inline (single self-contained HTML file).

2. **Use weasyprint** to convert:
```bash
weasyprint /home/jon/code/life-skills-advocates/life-skills-advocates-notes/summary-pdf/report.html /home/jon/code/life-skills-advocates/life-skills-advocates/life-skills-advocates-notes/summary-pdf/coaching-preparation-report.pdf
```

3. **DESIGN SYSTEM** — Use this professional color palette and typography:
```css
:root {
  --primary: #1e3a5f;      /* Deep navy */
  --primary-light: #2d5f8a; /* Medium blue */
  --accent: #c67b2e;        /* Warm amber/gold */
  --accent-light: #e8a84c;  /* Light amber */
  --text: #2d3748;          /* Dark charcoal */
  --text-light: #718096;   /* Medium gray */
  --bg-light: #f8fafc;      /* Very light gray */
  --bg-warm: #fffbf0;      /* Warm white */
  --success: #38a169;
  --danger: #e53e3e;
  --warning: #d69e2e;
  --white: #ffffff;
}
```

Font stack: `'Segoe UI', system-ui, -apple-system, sans-serif` (uses system fonts, no special installs needed)
Heading font: `Georgia, 'Times New Roman', serif` (for section titles — adds professional contrast)

4. **TITLE PAGE** — Must look like a real cover page:
   - Full-page dark background (gradient from var(--primary) to var(--primary-light))
   - Top: A decorative geometric pattern using CSS — create a row of small diamond shapes or dots across the top using CSS pseudo-elements or repeated backgrounds. Example:
     ```css
     .cover-pattern {
       background-image: repeating-linear-gradient(90deg, rgba(255,255,255,0.03) 0px, rgba(255,255,255,0.03) 1px, transparent 1px),
                           repeating-linear-gradient(0deg, rgba(255,255,255,0.03) 0px, rgba(255,255,255,0.03) 1px, transparent 1px);
     }
     ```
   - Center-aligned content vertically and horizontally using flexbox:
     - Small uppercase label: "EXECUTIVE FUNCTION COACHING" (letter-spacing: 4px, small, light opacity)
     - Horizontal rule (thin white/gold line, 200px wide)
     - Large title: "Coaching Preparation Report" (white, 32px, Georgia serif, bold)
     - Horizontal rule
     - Name: "Jon Penwood" (white, 22px, semi-bold)
     - Subtitle: "Prepared for Life Skills Advocate" (white, 14px, lighter opacity)
     - Date: "May 2026" (small, lighter opacity)
   - Add a decorative gold/amber accent line somewhere (2-3px thick, 300px wide)

5. **TABLE OF CONTENTS** — Clickable and professional:
   - Use HTML anchor links (`<a href="#section-name">`) and corresponding `id` attributes on sections
   - Light background (var(--bg-light))
   - Clean list styling with NO bullets (list-style: none)
   - Each entry: blue link text + dot leaders + page number
   - Example CSS for dot leaders:
     ```css
     .toc-entry { display: flex; justify-content: space-between; align-items: baseline; padding: 8px 0; border-bottom: 1px dotted #cbd5e0; }
     .toc-page { color: var(--text-light); font-size: 14px; flex-shrink: 0; }
     .toc-link { color: var(--primary-light); text-decoration: none; font-size: 15px; }
     .toc-link:hover { color: var(--accent); }
     ```
   - The page numbers should be on the right side of each entry

6. **CSS-BASED DIAGRAMS** — This is where HTML/CSS shines. Use CSS flexbox/grid for layout:

   a) **Challenge Cycle Diagram**: Circular flow using CSS flexbox:
   ```css
   .cycle-container { display: flex; flex-direction: column; align-items: center; gap: 8px; }
   .cycle-step { background: var(--bg-light); border-left: 3px solid var(--primary); padding: 12px 16px; border-radius: 4px; }
   .cycle-arrow { color: var(--primary-light); font-size: 20px; }
   ```
   Steps: "Strong Start" → "Sustained Effort (0.5-3 days)" → "Momentum Drops" → "Recovery Period" → "Cycle Repeats"
   
   b) **Morning Routine Flow**: Vertical flowchart:
   ```css
   .flow-step { background: var(--white); border: 1px solid #e2e8f0; border-radius: 6px; padding: 10px 16px; display: flex; align-items: center; gap: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.06); }
   .flow-arrow { text-align: center; color: var(--primary-light); font-size: 16px; }
   .flow-time { color: var(--text-light); font-size: 11px; margin-left: auto; }
   ```
   Steps with time estimates. Arrow dividers between steps.

   c) **Weekly Chore Calendar**: CSS Grid (7 columns):
   ```css
   .chore-grid { display: grid; grid-template-columns: repeat(7, 1fr); gap: 4px; }
   .chore-cell { background: var(--bg-light); padding: 8px 4px; border-radius: 4px; text-align: center; font-size: 12px; }
   .chore-header { background: var(--primary); color: white; font-weight: 600; font-size: 11px; text-transform: uppercase; }
   .chore-active { background: var(--primary-light); color: white; }
   ```
   
   d) **Financial Systems Flow**: Horizontal flow diagram using CSS flexbox:
   ```css
   .systems-flow { display: flex; gap: 8px; align-items: stretch; }
   .system-box { flex: 1; background: var(--bg-light); border-top: 3px solid var(--primary); padding: 12px; border-radius: 0 0 4px 4px; }
   .system-arrow { display: flex; align-items: center; color: var(--primary); font-size: 18px; }
   ```
   Boxes: "Income" → "Budget System" → "Bill Automation" → "Savings Allocation" → "Financial Awareness"

   e) **Priority Timeline**: Horizontal timeline with CSS:
   ```css
   .timeline { display: flex; position: relative; padding: 20px 0; }
   .timeline::before { content: ''; position: absolute; top: 50%; left: 0; right: 0; height: 3px; background: var(--primary-light); transform: translateY(-50%); }
   .timeline-event { position: relative; display: flex; flex-direction: column; align-items: center; z-index: 1; }
   .timeline-dot { width: 14px; height: 14px; border-radius: 50%; border: 3px solid; margin-bottom: 4px; }
   .timeline-label { font-size: 11px; text-align: center; max-width: 80px; }
   ```
   Color-code dots: red for ASAP, yellow for time-bound, orange for high, green for medium

   f) **Three-Phase Roadmap**: Horizontal flow with 3 large boxes:
   ```css
   .phase-container { display: flex; gap: 0; }
   .phase-box { flex: 1; background: var(--white); border: 1px solid #e2e8f0; padding: 16px; }
   .phase-box:first-child { border-radius: 8px 0 0 8px; }
   .phase-box:last-child { border-radius: 0 8px 8px 0; }
   .phase-number { background: var(--primary); color: white; width: 28px; height: 28px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: 700; font-size: 14px; margin-bottom: 8px; }
   .phase-arrow { display: flex; align-items: center; justify-content: center; background: var(--accent); color: white; width: 30px; min-height: 100%; font-size: 18px; }
   ```

7. **SECTION STYLING**:
   - Each section starts with a colored header bar:
     ```css
     .section-header { background: linear-gradient(90deg, var(--primary) 0%, var(--primary-light) 100%); color: var(--white); padding: 12px 20px; margin-bottom: 20px; border-radius: 6px; }
     .section-header h2 { margin: 0; font-family: Georgia, serif; font-size: 20px; }
     ```
   - Subsection headers: left-border accent style
     ```css
     .subsection { border-left: 3px solid var(--accent); padding-left: 16px; margin: 20px 0; }
     .subsection h3 { color: var(--primary); margin: 0 0 4px 0; font-size: 16px; }
     ```

8. **TABLE STYLING**:
   - Professional clean tables with alternating rows:
     ```css
     table { width: 100%; border-collapse: collapse; margin: 12px 0; font-size: 13px; }
     th { background: var(--primary); color: white; padding: 8px 12px; text-align: left; font-weight: 600; font-size: 12px; }
     td { padding: 8px 12px; border-bottom: 1px solid #e2e8f0; }
     tr:nth-child(even) td { background: var(--bg-light); }
     ```

9. **QUOTE/CALLOUT BOXES**:
   ```css
   .callout { background: var(--bg-warm); border-left: 4px solid var(--accent); padding: 12px 16px; border-radius: 0 4px 4px 0; margin: 12px 0; font-style: italic; color: var(--text); }
   ```

10. **PAGE SETUP** using weasyprint @page rules:
    ```css
    @page { size: letter; margin: 0.75in; @bottom-center { content: "Page " counter(page) " of " counter(pages); font-size: 9px; color: var(--text-light); } }
    @page :first { @bottom-center { content: none; } } /* no footer on cover */
    ```

11. **PAGE BREAKS** between sections:
    ```css
    .page-break { page-break-before: always; }
    ```

12. **CONTENT — ALL SECTIONS** (in this order):

    **Section 1: Coach Overview** (id="coach-overview")
    - "Who Jon Is" paragraph (rewrite of the professional version):
      "Jon Penwood is a 29-year-old recently divorced Senior AI Engineer. While he is highly capable in his career and hobbies — with deep technical skills and creative pursuits — he has long recognized gaps in other areas of executive function. He has spent years consuming self-help books, courses, and content, and has done extensive inner work through therapy. He is now at a point where the gap between understanding and execution is the central challenge. The theory is not the problem — it is the consistent application of that theory in daily life."
    - Core challenges as a styled list (NOT a table):
      - "Effort sustainability: Maintains focus for 0.5-3 days before momentum fades"
      - "Habit tracking: Uses an app called Car to track daily habits and routines"
      - "Overwhelm cycles: Competing responsibilities lead to avoidance and recovery loops"
      - "Task initiation: Difficulty starting tasks that feel large, even when they are not"
      - "Financial awareness: Wants to build better budgeting systems and financial follow-through"
    - Challenge Cycle diagram (CSS-based circular flow, see 6a above)
    - What Jon Is Looking For (short paragraph, non-prescriptive):
      "Jon is looking for practical, structured support with executive function and life skills — someone to help bridge the gap between knowing what to do and actually doing it consistently. He brings self-awareness about his patterns and genuine motivation for change."

    **Section 2: Daily Habits & Routines** (id="daily-habits")
    - Goals as a clean table (Goal, Frequency, Duration)
    - Morning Routine Flow diagram (CSS vertical flowchart, see 6b)
    - Evening Chain as a simpler horizontal flow
    - Keep it concise — this is a summary PDF

    **Section 3: Household Management** (id="household")
    - Tasks table (Task, Frequency, Day)
    - Weekly Chore Calendar grid (CSS 7-column grid, see 6c)
    - Key insight callout box

    **Section 4: Financial Situation** (id="finances")
    - NO specific dollar amounts
    - Abstract framing:
      - "Building budgeting systems and financial awareness"
      - "Developing consistent follow-through on financial tasks"
      - "A significant financial obligation ending mid-2026 creates an opportunity to redirect resources"
    - Financial Systems Flow diagram (CSS horizontal flow, see 6d)
    - Key focus areas as styled list: budget creation, bill automation, savings allocation, financial awareness building

    **Section 5: One-Off Tasks** (id="one-off-tasks")
    - Priority table (Task, Deadline, Urgency, First Step) — use color-coded urgency indicators:
      - Critical: red text or red left-border
      - Time-bound: amber/gold text
      - High: orange text
      - Medium: green text
    - Task Clusters as styled boxes (Financial, Healthcare, DMV, Home, Social)
    - Priority Timeline diagram (CSS horizontal, see 6e)

    **Section 6: Long-Term Goals** (id="long-term-goals")
    - Status: "Deferred until approximately 2 months of daily life stability"
    - Three-Phase Roadmap diagram (CSS horizontal flow, see 6f) with $130K asset target in Phase 3
    - Business philosophy paragraph (creative expression, not traditional startup milestones)
    - Milestone table (Horizon, Goal, Target)

    **Section 7: Understanding LSA's Coaching Approach** (id="lsa-approach")
    - This replaces the old prescriptive meeting plan
    - Intro: "This section reflects Jon's current understanding of LSA's coaching model based on their published materials."
    - LSA's 6-step framework (displayed as a horizontal flow or numbered boxes with descriptions):
      1. Check-in  2. Recap  3. Formulate  4. Skill Building  5. Planning  6. Accountability
    - With quote/callout: "Adapted from lifeskillsadvocate.com/discover-the-life-skills-advocate-difference/"
    - LSA's 9 adult support areas as a styled grid (2-3 columns of styled boxes):
      - Burnout/overwhelm, Neurodivergent strengths, Confidence at work, Personal relationships, Career transitions, Self-criticism cycle, Procrastination/perfectionism, Competing priorities, Self-trust rebuilding
    - Advocate360 platform mention ($10/mo — "includes EF Assessment, Goal Generator, and Skill Building Library")
    - Closing: "Jon looks forward to learning how the coaching relationship will actually be structured. This understanding is based on published materials and may differ from the actual experience."
    - Include clickable links to LSA website where appropriate (they should work in the PDF)

    **Section 8: Talking Points** (id="talking-points")
    - 6 core messages as styled callout boxes (NOT in a table — each message gets its own highlighted box):
      1. "I understand the theory behind all of this. What I need help with is actually doing it."
      2. "My pattern is I sustain effort for 0.5 to 3 days, then it slips away. That is the cycle I want to understand and work with."
      3. "I value having concrete takeaways from sessions that I can apply in daily life."
      4. "I am open to how between-session support works and what structure makes sense."
      5. "I am not looking for someone to fix me or replace my therapist. I want practical, structured support."
      6. "I already do inner work with a therapist. This is about building outer systems and daily life skills."
    - Remove all prescriptive talking points (no per-session demands)
    - "Key Considerations" box: Amy is the intake coordinator (not a coach), session flexibility exists, LSA doesn't coach to be neurotypical

13. **WHITESPACE MANAGEMENT**:
    - Use CSS `margin` and `padding` deliberately — don't over-space
    - 12px between major elements, 8px between related items, 4px between tight items
    - Sections should fill their pages reasonably well — use CSS to make content flow naturally rather than having huge gaps
    - If a section is short, it can share a page with the next section (don't force page breaks for short content)

14. **FINAL CHECKS**:
    - Run the weasyprint command and verify the PDF generates without errors
    - The PDF should be approximately 12-16 pages
    - All section IDs must match TOC href targets
    - No emojis, no Unicode issues
    - No financial numbers
    - No body doubling mentions
    - "Car" not "Habitica"
    - Non-prescriptive tone throughout

MUST NOT DO:
- Do NOT use reportlab — use weasyprint only
- Do NOT create images or use external assets — all graphics are CSS-only
- Do NOT use Unicode subscripts/superscripts
- Do NOT include emojis
- Do NOT be prescriptive about coaching
- Do NOT include specific financial numbers
- Do NOT mention body doubling
- Do NOT mention Habitica
- Do NOT modify files outside summary-pdf/
- Do NOT use a separate CSS file — embed all styles in the HTML file

CONTEXT:
- Output: /home/jon/code/life-skills-advocates/life-skills-advocates-notes/summary-pdf/coaching-report.html
- weasyprint 68.1 is installed
- Python 3.14 available
- Convert with: weasyprint input.html output.pdf
<!-- OMO_INTERNAL_INITIATOR -->