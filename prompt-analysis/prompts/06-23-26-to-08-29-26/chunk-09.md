# chunk-09 — prompts 401-450 of 1000
# span 06-23-26-to-08-29-26

## [0401] 07-27 16:49

(2026-07-27T16:49:10)
imgs should include all images from the book that can be found online for diagrams

## [0402] 07-27 16:51

(2026-07-27T16:51:09)
[CONTEXT] Building image folder for each Alex Hormozi book ($100M Offers, $100M Leads, $100M Money Models, Gym Launch Secrets, $100M Lost Chapters). Already have covers from official sources. Need to find FRAMEWORK DIAGRAMS that visualize concepts from inside the books.

[GOAL] Find URLs for downloadable image files (.jpg/.png/.webp) showing visualizations of Hormozi's frameworks. These appear in his YouTube videos, podcast thumbnails, Twitter threads, blog posts, course materials, and fan summaries.

[DOWNSTREAM] Will curl URLs into ./hermozi/{book}/imgs/ folders organized by book.

[REQUEST] For EACH framework below, find 1-3 direct image URLs (no Pinterest, no pages behind login):

1. Value Equation diagram (Dream Outcome × Likelihood / Time × Effort)
2. Grand Slam Offer diagram / framework visualization
3. Core Four 2x2 matrix (Warm/Cold × 1-to-1/1-to-Many)
4. Rule of 100 visualization
5. Lead Magnet examples
6. Trim & Stack matrix (Value × Cost 2x2)
7. Delivery Cube 6 dimensions
8. MAGIC Formula breakdown
9. Wedding Cake Gym Profit Model (3 tiers)
10. C-L-O-S-E-R sales script
11. 3 Macro Profit Levers (Price/Capacity/Overhead)
12. Money Model 3-stage flowchart (Get Cash → Get More Cash → Get Most Cash)
13. 30-Day Cash Cycle diagram
14. Performance Diamond (Avatar/Hook/Promise/Price)
15. Lead Generation Scrambler

Sources to try:
- YouTube thumbnail images (img.youtube.com/vi/{video_id}/maxresdefault.jpg) — search YouTube for "Hormozi {framework}"
- Twitter/X image hosts (pbs.twimg.com)
- LinkedIn blog posts about Hormozi frameworks
- Medium.com summary articles with embedded images
- Substack summaries
- SlideShare / SpeakerDeck decks
- Acquisition.com case study PDFs

Skip: Amazon (blocks), Pinterest, Instagram (login-required), TikTok.

Return format: framework_name | direct_image_url | source_description (per line)
<!-- OMO_INTERNAL_INITIATOR -->

## [0403] 07-27 16:51

Called the Read tool with the following input: {"filePath":"/home/jon/code/business/hermozi/100m-offers/imgs/framework-value-equation.webp"}

## [0404] 07-27 16:51

(2026-07-27T16:51:52)
Analyze this file and extract the requested information.

Goal: What does this image show? Is it a Value Equation framework diagram with formula components (Dream Outcome, Perceived Likelihood, Time Delay, Effort & Sacrifice)?

Provide ONLY the extracted information that matches the goal.
Be thorough on what was requested, concise on everything else.
If the requested information is not found, clearly state what is missing.

## [0405] 07-27 16:51

Called the Read tool with the following input: {"filePath":"/home/jon/code/business/hermozi/100m-leads/imgs/framework-lead-magnets.jpg"}

## [0406] 07-27 16:51

(2026-07-27T16:51:53)
Analyze this file and extract the requested information.

Goal: What does this image show? Is it a Lead Magnets framework diagram from $100M Leads?

Provide ONLY the extracted information that matches the goal.
Be thorough on what was requested, concise on everything else.
If the requested information is not found, clearly state what is missing.

## [0407] 07-27 16:51

(2026-07-27T16:51:53)
Analyze this file and extract the requested information.

Goal: What does this image show? Is it a roadmap framework diagram showing business scaling levels?

Provide ONLY the extracted information that matches the goal.
Be thorough on what was requested, concise on everything else.
If the requested information is not found, clearly state what is missing.

## [0408] 07-27 16:51

Called the Read tool with the following input: {"filePath":"/home/jon/code/business/hermozi/100m-leads/imgs/framework-roadmap.jpg"}

## [0409] 07-27 16:51

Called the Read tool with the following input: {"filePath":"/home/jon/code/business/hermozi/100m-offers/imgs/workshop.jpg"}

## [0410] 07-27 16:51

(2026-07-27T16:51:54)
Analyze this file and extract the requested information.

Goal: What is shown in this image? Is it a Hormozi framework diagram or something else?

Provide ONLY the extracted information that matches the goal.
Be thorough on what was requested, concise on everything else.
If the requested information is not found, clearly state what is missing.

## [0411] 07-27 16:52

(2026-07-27T16:52:28)
[CONTEXT] Downloaded 24 images from acquisition.com organized into ./hermozi/{book}/imgs/ folders. Need to verify each "framework-*.jpg" image actually shows a Hormozi framework diagram (not a generic stock photo or portrait).

[GOAL] For each image below, describe what it shows in 1-2 sentences. Confirm if it's a framework/infographic/diagram vs. just a photo/text cover.

[REQUEST] Look at each of these files and tell me what's in them:

1. /home/jon/code/business/hermozi/100m-offers/imgs/framework-value-equation.webp
2. /home/jon/code/business/hermozi/100m-offers/imgs/workshop.jpg
3. /home/jon/code/business/hermozi/100m-offers/imgs/workshop-flicks.jpg
4. /home/jon/code/business/hermozi/100m-leads/imgs/framework-lead-magnets.jpg
5. /home/jon/code/business/hermozi/100m-leads/imgs/framework-lead-magnets-v2.jpg
6. /home/jon/code/business/hermozi/100m-leads/imgs/framework-cold-outreach.jpg
7. /home/jon/code/business/hermozi/100m-leads/imgs/framework-content-method.jpg
8. /home/jon/code/business/hermozi/100m-leads/imgs/framework-paid-ads.jpg
9. /home/jon/code/business/hermozi/100m-leads/imgs/framework-referrals.jpg
10. /home/jon/code/business/hermozi/100m-leads/imgs/framework-affiliates.jpg
11. /home/jon/code/business/hermozi/100m-leads/imgs/framework-first-5-clients.jpg
12. /home/jon/code/business/hermozi/100m-leads/imgs/framework-roadmap.jpg

Return format per image:
- FILE: <filename>
- TYPE: framework_diagram | text_screenshot | portrait | stock_photo | cover_art | other
- CONTENT: 1-2 sentence description
- KEEP: yes | no (yes if it's a real diagram/chart/screenshot of book content)
<!-- OMO_INTERNAL_INITIATOR -->

## [0412] 07-27 16:53

(2026-07-27T16:53:35)
You need to analyze 12 images to determine which are actual framework diagrams/infographics vs. generic photos/cover art.

**Context:** Downloaded 24 images from acquisition.com organized into ./hermozi/{book}/imgs/ folders. Need to verify each "framework-*.jpg" image actually shows a Hormozi framework diagram (not a generic stock photo or portrait).

**Goal:** For each image below, describe what it shows in 1-2 sentences. Confirm if it's a framework/infographic/diagram vs. just a photo/text cover.

**Images to analyze:**

1. /home/jon/code/business/hermozi/100m-offers/imgs/framework-value-equation.webp
2. /home/jon/code/business/hermozi/100m-offers/imgs/workshop.jpg
3. /home/jon/code/business/hermozi/100m-offers/imgs/workshop-flicks.jpg
4. /home/jon/code/business/hermozi/100m-leads/imgs/framework-lead-magnets.jpg
5. /home/jon/code/business/hermozi/100m-leads/imgs/framework-lead-magnets-v2.jpg
6. /home/jon/code/business/hermozi/100m-leads/imgs/framework-cold-outreach.jpg
7. /home/jon/code/business/hermozi/100m-leads/imgs/framework-content-method.jpg
8. /home/jon/code/business/hermozi/100m-leads/imgs/framework-paid-ads.jpg
9. /home/jon/code/business/hermozi/100m-leads/imgs/framework-referrals.jpg
10. /home/jon/code/business/hermozi/100m-leads/imgs/framework-affiliates.jpg
11. /home/jon/code/business/hermozi/100m-leads/imgs/framework-first-5-clients.jpg
12. /home/jon/code/business/hermozi/100m-leads/imgs/framework-roadmap.jpg

**Return format per image:**
- FILE: <filename>
- TYPE: framework_diagram | text_screenshot | portrait | stock_photo | cover_art | other
- CONTENT: 1-2 sentence description
- KEEP: yes | no (yes if it's a real diagram/chart/screenshot of book content)

**IMPORTANT:**
- Read each image carefully using your visual analysis capabilities
- Be precise about TYPE - distinguish between actual framework diagrams (charts, flowcharts, infographics with structured information) versus generic images
- KEEP should be "yes" ONLY if the image contains actual book content: framework, diagram, screenshot of text/content, structured information
- KEEP should be "no" for: portraits, generic stock photos, book covers without framework, promotional images without substantive content

Analyze all 12 images and return results in the exact format specified.

## [0413] 07-27 16:54

(2026-07-27T16:54:39)
[CONTEXT] Building ./hermozi/ image library. Batch 2 contains 20 images staged at /tmp/hormozi-bulk2/. These are UUID-named files scraped from acquisition.com archive.org snapshots. Need to classify each so I can route them to the correct book subfolder.

[GOAL] For each image: describe what it shows in 1-2 sentences, then classify into ONE of:
- "framework-diagram" (visualization of a Hormozi concept: charts, flowcharts, matrices, funnels, equations)
- "framework-thumbnail" (YouTube/podcast thumbnail that likely discusses a framework)
- "book-cover" (cover of a Hormozi book)
- "portrait" (Alex, Leila, or other person)
- "stock-photo" (generic photo, no info value)
- "ui-screenshot" (webpage screenshot, dashboard)
- "logo" (branding/icon)
- "other" (explain)

[DOWNSTREAM] Files classified as framework-diagram, framework-thumbnail, or book-cover will be moved to book folders. Portraits/stock/logos will be deleted.

[REQUEST] Look at each file and return JSON array:
[
  {"file": "/tmp/hormozi-bulk2/6217bc0764d036651ef36dd1.jpeg", "size": "3024x4032", "description": "...", "classification": "..."},
  ...
]

Files (sorted by size desc, with dimensions):
1. /tmp/hormozi-bulk2/6217bc0764d036651ef36dd1.jpeg (3024x4032, 2.6MB) - largest, likely important
2. /tmp/hormozi-bulk2/1.png (1080x1080, 529KB)
3. /tmp/hormozi-bulk2/62d28240-deac-4ef7-935b-72baaad54c72.png (452x558, 514KB)
4. /tmp/hormozi-bulk2/b961464c-c217-486f-b597-6b0e92ac0450.png (552x421, 428KB)
5. /tmp/hormozi-bulk2/0a36560b-3ea6-460b-ab43-0192a9d0ee5c.png (356x604, 287KB)
6. /tmp/hormozi-bulk2/5891e0d8-2475-4783-b69c-b70d9f467a42.png (329x342, 204KB)
7. /tmp/hormozi-bulk2/eee97622-026c-447d-b658-6dfc725ebb7f.png (1920x1080, 198KB)
8. /tmp/hormozi-bulk2/3a30fd34-52ed-4586-b72a-b7ffbb5416ef.png (253x299, 170KB)
9. /tmp/hormozi-bulk2/32880030-1678-41d7-94ef-8e3817890705.png (1920x1080, 136KB)
10. /tmp/hormozi-bulk2/747cc555-8cc5-42f6-a834-f5a5becd3050.png (297x274, 93KB)
11. /tmp/hormozi-bulk2/4853ac01-5e3d-4aa6-ad9a-4ae0831771f1.png (422x278, 73KB)
12. /tmp/hormozi-bulk2/f6c6213b-4427-4772-9164-b453687162ea.png (232x284, 65KB)
13. /tmp/hormozi-bulk2/e00b1815-d23e-4821-a809-ac953291aa9f.png (1920x1080, 61KB)
14. /tmp/hormozi-bulk2/6844586a-da8e-4955-a3f3-ad88dba636dd.png (363x257, 53KB)
15. /tmp/hormozi-bulk2/c320bbb4-a7dc-4398-9747-9d3d502d70de.png (511x300, 43KB)
16. /tmp/hormozi-bulk2/AlexThreads.png (1080x1080, 41KB) - filename suggests portrait
17. /tmp/hormozi-bulk2/LeilaThreads.png (1080x1080, 38KB) - filename suggests portrait
18. /tmp/hormozi-bulk2/2e58a134-f546-4bea-8723-f9d09d49c1c5.png (435x227, 22KB)
19. /tmp/hormozi-bulk2/26095791-1aa4-496c-ac4d-a36c2f22ce09.png (1920x1080, 20KB)
20. /tmp/hormozi-bulk2/88c599db-973a-43b4-a16d-549c9ae1e0ec.png (1101x197, 18KB)

Be concise but accurate. Output as JSON array only.
<!-- OMO_INTERNAL_INITIATOR -->

## [0414] 07-27 16:55

(2026-07-27T16:55:27)
Examine all 20 images in /tmp/hormozi-bulk2/ and classify each one.

For each image, provide:
1. File path (from the list below)
2. Size (provided in the list)
3. 1-2 sentence description of what the image shows
4. Classification into ONE of these categories:
   - "framework-diagram" (visualization of a Hormozi concept: charts, flowcharts, matrices, funnels, equations)
   - "framework-thumbnail" (YouTube/podcast thumbnail that likely discusses a framework)
   - "book-cover" (cover of a Hormozi book)
   - "portrait" (Alex, Leila, or other person)
   - "stock-photo" (generic photo, no info value)
   - "ui-screenshot" (webpage screenshot, dashboard)
   - "logo" (branding/icon)
   - "other" (explain briefly)

Files to analyze (sorted by size desc):
1. /tmp/hormozi-bulk2/6217bc0764d036651ef36dd1.jpeg (3024x4032)
2. /tmp/hormozi-bulk2/1.png (1080x1080)
3. /tmp/hormozi-bulk2/62d28240-deac-4ef7-935b-72baaad54c72.png (452x558)
4. /tmp/hormozi-bulk2/b961464c-c217-486f-b597-6b0e92ac0450.png (552x421)
5. /tmp/hormozi-bulk2/0a36560b-3ea6-460b-ab43-0192a9d0ee5c.png (356x604)
6. /tmp/hormozi-bulk2/5891e0d8-2475-4783-b69c-b70d9f467a42.png (329x342)
7. /tmp/hormozi-bulk2/eee97622-026c-447d-b658-6dfc725ebb7f.png (1920x1080)
8. /tmp/hormozi-bulk2/3a30fd34-52ed-4586-b72a-b7ffbb5416ef.png (253x299)
9. /tmp/hormozi-bulk2/32880030-1678-41d7-94ef-8e3817890705.png (1920x1080)
10. /tmp/hormozi-bulk2/747cc555-8cc5-42f6-a834-f5a5becd3050.png (297x274)
11. /tmp/hormozi-bulk2/4853ac01-5e3d-4aa6-ad9a-4ae0831771f1.png (422x278)
12. /tmp/hormozi-bulk2/f6c6213b-4427-4772-9164-b453687162ea.png (232x284)
13. /tmp/hormozi-bulk2/e00b1815-d23e-4821-a809-ac953291aa9f.png (1920x1080)
14. /tmp/hormozi-bulk2/6844586a-da8e-4955-a3f3-ad88dba636dd.png (363x257)
15. /tmp/hormozi-bulk2/c320bbb4-a7dc-4398-9747-9d3d502d70de.png (511x300)
16. /tmp/hormozi-bulk2/AlexThreads.png (1080x1080)
17. /tmp/hormozi-bulk2/LeilaThreads.png (1080x1080)
18. /tmp/hormozi-bulk2/2e58a134-f546-4bea-8723-f9d09d49c1c5.png (435x227)
19. /tmp/hormozi-bulk2/26095791-1aa4-496c-ac4d-a36c2f22ce09.png (1920x1080)
20. /tmp/hormozi-bulk2/88c599db-973a-43b4-a16d-549c9ae1e0ec.png (1101x197)

Output as a valid JSON array only, like this:
[
  {"file": "/tmp/hormozi-bulk2/6217bc0764d036651ef36dd1.jpeg", "size": "3024x4032", "description": "...", "classification": "..."},
  ...
]

Be concise but accurate. Return JSON only, no markdown code blocks.

## [0415] 07-27 16:56

(2026-07-27T16:56:34)
Copy all 20 images from /tmp/hormozi-bulk2/ to /home/jon/.local/share/opencode/tool-output/hormozi-bulk2/

First create the destination directory if it doesn't exist, then copy all files.

Use: cp -r /tmp/hormozi-bulk2/ /home/jon/.local/share/opencode/tool-output/

Return the list of files copied and their sizes.

## [0416] 07-27 16:57

(2026-07-27T16:57:28)
[CONTEXT] Building ./hermozi/ image library. Need direct YouTube video IDs (11-char like dQw4w9WgXcQ) for videos on Alex Hormozi's main channel (@AlexHormozi / @TheGameAlexHormozi) that discuss these frameworks. Will fetch thumbnails via https://img.youtube.com/vi/{ID}/maxresdefault.jpg.

[GOAL] Return ONLY direct 11-char YouTube video IDs for videos covering these specific frameworks. Format: framework_name|videoID|title.

[DOWNSTREAM] Will download maxresdefault.jpg for each.

[REQUEST] Use websearch for each framework name + "Alex Hormozi" + "youtube.com/watch" to find video URLs. Then extract the video ID (last 11 chars after ?v= or youtu.be/). Return as many as you can find, but ONLY verifiable IDs from real YouTube URLs in search results. Do NOT fabricate IDs.

Frameworks to find (priority order):
1. Value Equation (Hormozi)
2. Grand Slam Offer
3. Trim and Stack
4. Delivery Cube
5. MAGIC Naming Formula
6. Core Four lead generation
7. Lead Magnet
8. Rule of 100
9. CLOSER sales script
10. Money Model / $100M Money Models
11. Wedding Cake gym model
12. Performance Diamond
13. Lead Generation Scrambler
14. 30-Day Cash Cycle
15. God Mode Guarantees
16. Scarcity Stack

Also search for: "$100M Offers full audiobook", "$100M Leads summary", "$100M Money Models book review".

Return as JSON array: [{"framework": "...", "video_id": "abc123xyz-0", "title": "..."}]
<!-- OMO_INTERNAL_INITIATOR -->

## [0417] 07-27 16:57

(2026-07-27T16:57:39)
Examine all 20 images in /home/jon/.local/share/opencode/tool-output/hormozi-bulk2/ and classify each one.

For each image, provide:
1. File path (use the full path from the new location)
2. Size (get actual dimensions and file size)
3. 1-2 sentence description of what the image shows
4. Classification into ONE of these categories:
   - "framework-diagram" (visualization of a Hormozi concept: charts, flowcharts, matrices, funnels, equations)
   - "framework-thumbnail" (YouTube/podcast thumbnail that likely discusses a framework)
   - "book-cover" (cover of a Hormozi book)
   - "portrait" (Alex, Leila, or other person)
   - "stock-photo" (generic photo, no info value)
   - "ui-screenshot" (webpage screenshot, dashboard)
   - "logo" (branding/icon)
   - "other" (explain briefly)

Output as a valid JSON array only, like this:
[
  {"file": "/home/jon/.local/share/opencode/tool-output/hormozi-bulk2/6217bc0764d036651ef36dd1.jpeg", "size": "3024x4032", "description": "...", "classification": "..."},
  ...
]

Be concise but accurate. Return JSON only, no markdown code blocks.

## [0418] 07-27 16:59

(2026-07-27T16:59:15)
Analyze these 4 images to determine if they are actual framework diagrams vs. generic photos:

1. /home/jon/code/business/hermozi/100m-offers/imgs/framework-value-equation.webp
2. /home/jon/code/business/hermozi/100m-offers/imgs/workshop.jpg
3. /home/jon/code/business/hermozi/100m-offers/imgs/workshop-flicks.jpg
4. /home/jon/code/business/hermozi/100m-leads/imgs/framework-lead-magnets.jpg

Return format per image:
- FILE: <filename>
- TYPE: framework_diagram | text_screenshot | portrait | stock_photo | cover_art | other
- CONTENT: 1-2 sentence description
- KEEP: yes | no (yes only if real diagram/chart/screenshot of book content)

Be precise: KEEP = yes ONLY for actual framework/diagram/screenshot with structured book content.

## [0419] 07-27 16:59

(2026-07-27T16:59:49)
Analyze these 4 images to determine if they are actual framework diagrams vs. generic photos:

1. /home/jon/code/business/hermozi/100m-offers/imgs/framework-value-equation.webp
2. /home/jon/code/business/hermozi/100m-offers/imgs/workshop.jpg
3. /home/jon/code/business/hermozi/100m-offers/imgs/workshop-flicks.jpg
4. /home/jon/code/business/hermozi/100m-leads/imgs/framework-lead-magnets.jpg

Return format per image:
- FILE: <filename>
- TYPE: framework_diagram | text_screenshot | portrait | stock_photo | cover_art | other
- CONTENT: 1-2 sentence description
- KEEP: yes | no (yes only if real diagram/chart/screenshot of book content)

Read each image and describe what's visible.

## [0420] 07-27 17:02

(2026-07-27T17:02:35)
Analyze these 4 images to determine if they are actual framework diagrams vs. generic photos:

5. /home/jon/code/business/hermozi/100m-leads/imgs/framework-lead-magnets-v2.jpg
6. /home/jon/code/business/hermozi/100m-leads/imgs/framework-cold-outreach.jpg
7. /home/jon/code/business/hermozi/100m-leads/imgs/framework-content-method.jpg
8. /home/jon/code/business/hermozi/100m-leads/imgs/framework-paid-ads.jpg

Return format per image:
- FILE: <filename>
- TYPE: framework_diagram | text_screenshot | portrait | stock_photo | cover_art | other
- CONTENT: 1-2 sentence description
- KEEP: yes | no (yes only if real diagram/chart/screenshot of book content)

Context from README: "Framework diagrams are not publicly available as downloadable images from official sources." Files named "framework-*" may be misnamed.

Use file analysis, dimensions, color distribution, and compare with other files in same directories to determine content type.

## [0421] 07-27 17:06

(2026-07-27T17:06:13)
Analyze these 4 images to determine if they are actual framework diagrams vs. generic photos:

9. /home/jon/code/business/hermozi/100m-leads/imgs/framework-referrals.jpg
10. /home/jon/code/business/hermozi/100m-leads/imgs/framework-affiliates.jpg
11. /home/jon/code/business/hermozi/100m-leads/imgs/framework-first-5-clients.jpg
12. /home/jon/code/business/hermozi/100m-leads/imgs/framework-roadmap.jpg

Return format per image:
- FILE: <filename>
- TYPE: framework_diagram | text_screenshot | portrait | stock_photo | cover_art | other
- CONTENT: 1-2 sentence description
- KEEP: yes | no (yes only if real diagram/chart/screenshot of book content)

Use same analysis method as previous batch: file properties, color distribution, structure scores, edge detection, brightness differences, center contrast, and compare with other framework images in the directory. Focus on distinguishing actual diagrams from misnamed generic images.

## [0422] 07-27 17:31

(2026-07-27T17:31:34)
delete all the images that are not diagrams from inside of each book that show concepts in the book

## [0423] 07-27 17:35

(2026-07-27T17:35:04)
[search-mode]
MAXIMIZE SEARCH EFFORT. Launch multiple background agents IN PARALLEL:
- explore agents (codebase patterns, file structures, ast-grep)
- librarian agents (remote repos, official docs, GitHub examples)
Plus direct tools: Grep, ripgrep (rg), ast-grep (sg)
NEVER stop at first result - be exhaustive.

[analyze-mode]
ANALYSIS MODE. Gather context before diving deep:
CONTEXT GATHERING (parallel):
- 1-2 explore agents (codebase patterns, implementations)
- 1-2 librarian agents (if external library involved)
- Direct tools: Grep, AST-grep, LSP for targeted searches

IF COMPLEX - DO NOT STRUGGLE ALONE. Consult specialists:
- **Oracle**: Conventional problems (architecture, debugging, complex logic)
- **Artistry**: Non-conventional problems (different approach needed)

SYNTHESIZE findings before proceeding.
---
MANDATORY delegate_task params: ALWAYS include load_skills=[] and run_in_background when calling delegate_task.
Example: delegate_task(subagent_type="explore", prompt="...", run_in_background=true, load_skills=[])

---

where does it talk about how to start and where is the summary .d2 diagram orm?

## [0424] 07-27 17:40

(2026-07-27T17:40:31)
make the orm a dag

## [0425] 07-27 17:50

(2026-07-27T17:50:11)
[analyze-mode]
ANALYSIS MODE. Gather context before diving deep:
CONTEXT GATHERING (parallel):
- 1-2 explore agents (codebase patterns, implementations)
- 1-2 librarian agents (if external library involved)
- Direct tools: Grep, AST-grep, LSP for targeted searches

IF COMPLEX - DO NOT STRUGGLE ALONE. Consult specialists:
- **Oracle**: Conventional problems (architecture, debugging, complex logic)
- **Artistry**: Non-conventional problems (different approach needed)

SYNTHESIZE findings before proceeding.
---
MANDATORY delegate_task params: ALWAYS include load_skills=[] and run_in_background when calling delegate_task.
Example: delegate_task(subagent_type="explore", prompt="...", run_in_background=true, load_skills=[])

---

change the dag to be horizontal and put the orm structures in the dag. I like the groupings so keep those.  after that I want you to critically review the horizontal dag with the orm structures and read through each summary and do web searches to make sure your not missing a single detail.  if you need to make more than one dag where there is one high level master dag and refrerences to sub dag diagrams with an infinite canvas type deal that would be helpful and add links to the svgs so I can view them in the browers like with this one 

data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz48c3ZnIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiIGRhdGEtZDItdmVyc2lvbj0idjAuNy4xIiBwcmVzZXJ2ZUFzcGVjdFJhdGlvPSJ4TWluWU1pbiBtZWV0IiB2aWV3Qm94PSIwIDAgNDc1MCA0NzYwIj48c3ZnIGNsYXNzPSJkMi00MTAyODc2MzM5IGQyLXN2ZyIgd2lkdGg9IjQ3NTAiIGhlaWdodD0iNDc2MCIgdmlld0JveD0iLTg5IC04OSA0NzUwIDQ3NjAiPjxyZWN0IHg9Ii04OS4wMDAwMDAiIHk9Ii04OS4wMDAwMDAiIHdpZHRoPSI0NzUwLjAwMDAwMCIgaGVpZ2h0PSI0NzYwLjAwMDAwMCIgcng9IjAuMDAwMDAwIiBmaWxsPSIjRkZGRkZGIiBjbGFzcz0iIGZpbGwtTjciIHN0cm9rZS13aWR0aD0iMCIgLz48c3R5bGUgdHlwZT0idGV4dC9jc3MiPjwhW0NEQVRBWwouZDItNDEwMjg3NjMzOSAudGV4dCB7Cglmb250LWZhbWlseTogImQyLTQxMDI4NzYzMzktZm9udC1yZWd1bGFyIjsKfQpAZm9udC1mYWNlIHsKCWZvbnQtZmFtaWx5OiBkMi00MTAyODc2MzM5LWZvbnQtcmVndWxhcjsKCXNyYzogdXJsKCJkYXRhOmFwcGxpY2F0aW9uL2ZvbnQtd29mZjtiYXNlNjQsZDA5R1JnQUJBQUFBQUJiMEFBb0FBQUFBSWt3QUFndUZBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUJQVXk4eUFBQUE5QUFBQUdBQUFBQmdYZC9WbzJOdFlYQUFBQUZVQUFBQXpnQUFBVFlJUVFvMFoyeDVaZ0FBQWlRQUFBK01BQUFWb0N5ejNUdG9aV0ZrQUFBUnNBQUFBRFlBQUFBMkc0VWUzMmhvWldFQUFCSG9BQUFBSkFBQUFDUUtoQVlMYUcxMGVBQUFFZ3dBQUFEdEFBQUJKSXFMRG81c2IyTmhBQUFTL0FBQUFKUUFBQUNVek83U3hHMWhlSEFBQUJPUUFBQUFJQUFBQUNBQVlRRDJibUZ0WlFBQUU3QUFBQU1qQUFBSUZBYkRWVTF3YjNOMEFBQVcxQUFBQUIwQUFBQWcvOUVBTWdBREFna0JrQUFGQUFBQ2lnSllBQUFBU3dLS0FsZ0FBQUZlQURJQkl3QUFBZ3NGQXdNRUF3SUNCR0FBQXZjQUFBQURBQUFBQUFBQUFBQkJSRUpQQUVBQUlQLy9BdTcvQmdBQUE5Z0JFU0FBQVo4QUFBQUFBZVlDbEFBQUFDQUFBM2ljaE0vSkxuTnhHTWZ4ejJuUDI3ZTBabXJtb0txdGVhNmhLRFlTZStuU1Vub0hMb3dkY1JYV3J1UXZUazVzUGV0UDhudStpT1JGS0l1akpxb1NzVmlpcG1IZGhrM2JXazZkYTd2em9PdkpzODhRU0ZYOVYrMWtxdU5lMTZQZWp3cGZTaW9xaGd3YmtaTlRWQWdmNFQyOGhkZndFbnJwL3Q4WDJYWGhVc2VadGl0NzloMDRkT1JZeTBtNmZldEdUbDdzbjRML2l2cjBLeWtiTUpoOU1Hck11QWtWazZaTW16RnJ6cndGaXhKTGxxMm9XbFd6cHE2aG1mVnRwWVhYZkFNQUFQLy9BUUFBLy8vS0JTTnFBQUI0bkl4NGUzQWIxM1grdVJkTGdCVEFCd2dzbHlBQkF0Z2xzTVNMSUxGWUxBbVFBSWtYSVJJa0tJQVVSVktpWHBRb1NwWWlVN0gxa3kxTHlVK3lwTml1Z3pqMjJDUEprcEo0cG5ack5YWXpJOGZqU2ROYXNVclhzZFVrcmgzSGtjYk5kQmhQN0NRTnk3UzFZeTQ2dXdBZnNwdVovb085ZjF6Y2M3N3ZmdWM3WnhmS1lCd0E4L2h4VUVBRlZFTXRrQUNjMXFxMVdWbVdVUW1jSURDVVFtQ1JWaldPZmlubUVkcm9Kd0lCb2ozNlVmVFl5Wk5veXduOCtQSmR3ZE16TTY5TkhUMHEvc1hDaDZJUHZmVWhZRkFBWUJQT1F3Vm9BWFFxanJYYldVYXBWT2c0SGNNeXFqZk1yNWxyTFRWRXRlVVh0NmR1ajRkL0gwRmZtcDRXRG5aMkhoUW5jSDc1OFB3OEFJQUNKZ0J3TTg2REZocUFrWExqZkhWMXBGNnBJdVdIa2xGd3ZnRHZ0ek9NZG1VeGNUMjJwN1BkR3hxSUhPNC9zWE9rZjNCd3o5em8xTGJOY3podlNRYmJNOVdFZWlqZXM5bUpqZ1Y5blczTFM1Rm9keWNBSVBBWGxuQWp2Z2dtZ0RMYWJ1ZjlnUURucTZOVWRqdERLNVdrdnE2Tzh3VUVTcWxFMmV4WEI5S25jMTFialo2R3FETzhqZk5OaHIzOTVsWjJsMmJUVXdmMlA1VnR0d1NNZE8rOTJleXhhQXZ0OS9nQUFNdFkvRGdQNVJJbk1oSlNyMlRZMWJ5ZmVlclMwMCtNRGh3NWN1VElBTTQvZC9IcHY0ay9mUHo0ZzNKdUV3RG9OczZEV3I0ZjBrcHlKRU5heVFsMG4vamVKNStnZHB4UHZ0WDN1NzdWdlcvTDNLL3QxY283UC8wVTU1TzNrK0l2VnZIYThVV3cvRG04RWx5ZTRUbXRVb20yYmo2VEhqbzNGdDltYkRWRWZkRmQvTjJ6VEkvdTRYZk5zeVhJWEZPZ29ibjMzdXp4YjVDMWY1MFFQN2E2U3JsZzMwcmVrckk0TGFPMWFpZHlxSDFrUkx5SjgrSnZrVzc1TU9MRk4xWnloNnM0TCtsSDJqK1JrOFJRd3ZSam5JZXkwam5rUkE2WmNYNzVKUWx5S2M0RE9DL2RIYWZsZEhWMUZCY0lDRG9wbWo4Z01Db0ZvMkNadWpwU096RjlRa05wQ0EycE9iNW5xRnhCK0k4THgvMkVRb1h6NHJmcEJFMG5hRFMxZkJqTnVnKzRuaENmUnlOUHVBNjR4U2RYWTNod0huVEZHQlJudC9NU25wV1ROLysyajFDb01wdC8xMGNRMG5uVDUzd0gvQ2kzZkJnOWZiWjluMTk4RHJETStSNThFYW8vcHpKWnpLd3ZJRXVCbHNXRzB0bVRmWDBuczdrVHFkU0pYR2lzYmYrV0xmdmJ0bWhHenMvT1BybHAwNU96cytkSE5zYU9aZTk3OU5IN3NzZGlzS294dGN5M2ZsMjFNSXgyclR4ZTZUOFVQblBYWGJzMjU4WTJUK0Y4ODJocVpscjhES1Y2azMyQ2ZJYS9NSW1QNEl0UUNVNEEyeDA1bGRGMnRoV3Z6MW0xVmhYcDFSeVJZdkkrVTk5ZFhjWHNnL3VTMUNoZlIxZldWUHM2WS9lUFNBbVAzQis3UFI3YnNPV3hIVVVzVS9seGpjdGRSdlFvSzFaeFdIQWVxb0JhWC9VNlJyRWV5cHV4ZmFIaCtGOU9YVHA2YURDYkhUeUU4OHltZUhxYlZ2d1ZJc1dQMEhpa3A5ZGYxSWV6c0lSK2p5K0NSMmFkRmVTc2ViL2R6cTdoS1NsZndrbFJUVmhDaDJvUzk3cDh6SGF1TjJWcU4wK1p1eDM4VkNnMHpYaWFOcllLTWF1dlladTl1emt3cmVIZFFac24xRWEzR0tzY2xjNW9teS9qOFRRSFRGYS8yK3hvVUxmVWVIcmIvYU0rUUdBRVFKL2hQS2drVkF4dkpSbnRyMjZnRDI3Zy9tUnkrVm94MTdIQ0VtN0ZlY21MWllWb09XM1Jld0x5VXFsRXNkaUJjTTZSY0xtVGp1SHdmazNnK0N6NnF2aEFadEp1bjh5Z1UrTEoyZU1CUUxJZ0ZEZ1BsUUNjWWwxTktINzYwL0haMmtZZFVXdlV6bzcrTTg2TGw0Sjdnc0U5UWJSTHJqVUtBSCtLODJEOTNQL1duY0FvaW42dVVuejc0YzJKQ24wRm9hNVhUNlluTmZVYW9xSjJRMkw0M1BUdWl1cHlRbFZidmhQbnhRdjhmcDQvNEVkN3hBditBOFhWOG1IMHNIMmozYjdSTHQ0RHFGQUFRTGR3SHFvQk9GN0JVYVZRQXFjZ2YvVGErR2lOdVliUTB0VWpZNitKNkxHWG01TTJXN0w1WmZHUVdOTHJFbm9STFVJRE5BTlF0RlJTZ2wrK1NoVXJYeXlwWmFSa1djbk9aR043dFh2VDF5OW9YUzNPZnBPRjNoMGNINDZyRlBTbU9pYk1ITnZwMDJ6c0hSN1ZtanNZaTc2enpuRndVbnduYUhSR2FmUFo2aTZ2d3dZWXNvVWw5Q2M4RDdxU2M3S01pdEZ5cEtvWXExakh4WktSdWhSeTBCc3RDbFUwaTYyWmx1MjdRdHVUWFpsUXd0ekRXQ0lhcThtSDUxL2RZbUxQM0oyN041eVltUmplVFZzS1JxcW9nOWJDRXZvdVdwUTA4K2Y5V1NxODJwNTlYYjBId20wSmc1UDBtdHdKTmhlamczWE4xbUZOMTl4d2RxNkxwZ0s2ZXU5b1IyN0dwQmRNVm9remIyRUp2YmVDb2NpWmZEakxjeXRrQ2Z4cW9QK2VQQlRhS1RqREZpSVhWeW1NYVVOUGw3bXppWTNZazVvSGoyV09oSnNhY3E4c2QzUWFIWW1ZYUtTOHVZNngzWURsL1A4SkxVSTltTzlBSUJXeWRkVTJGRmFaS2tUMTdnOUhwb1Z0ZXhBV3YxODJsbVJDalNaejVnMUVSRHE1VFpydXVjendYUGo0dmtwRHhlQldVaHZRTnlGNy8yQkc1cWtKQUVYdzI4WFpoZUVGM2wvaWlhRkp1ZmZ0aUVZVEd5bG5UVzJqTVQ0emc3NFRMaHZzSDZ0UVJUUlRnekZ4bXp4bmVBb1c5REZhaEhib2hzRlZGZkgyZFEvNVVJNWtTb01IelJidm9IVG5DdCthVGVwSy9rVGJpM3YrYy95dzNWcHJvSFgxckcra1hkOWMrZHkwbG1vYjlyRjBaYTJ0ZldwMHRPdFEydG5kNVhKMWRRZVNJNXgzcE1wYTAxQS84RUU4WXU2c0k5UXRSbk5ySmFHUHUvZ2hwNm9zVXNPYi9XbUhWdDJvcDVxRWJrL2FpMTZNOEh4WEY4OUh4SFBkZHJxQklIUk9rbTJWdWNrQ29IZnhmS2tickdoVTZvcXlQclhacklJWjlBMzJaZDF0dHBBTno3ODZiZlh1M0NiK0dEbmlZYnROdkFLRkFpUUE0SHY0R3JiTEt5VWtqOFBxMlF0NEhqVEZuc3pwT0pXT1lWVmtkcFBpNXVSM1hwNTRkQkxQaTAwSXJvdTNmclAvSzZYL0ZKYmc1M2hlcW5PSlk5bmFTa0o0cnRXUnJhb2dWQ3AxZVoybWs4ZDdseC9YYVJFS0UwUXhGdjREV3BROVNjdEozaURkeGgxb1ZLdlBiRnlsc0tSZEhaRnErNUI3WUdQVzNScUlaOTNlUUJ3dEpCbHZ1OXZoWDRFNElGNHBQVmE0UW9zbHJrb3gxbk1WVnltWW9WV3k1TVB1NEtxaytYOUhpMUFOamY5cmYxL1ZDS29PelVRaU02R3V2WkhJM3E3STRHQWtQRFJVcXRldXVlendYRmQ4SmpleWI5OUliZ1preitIUW45QmlxVjdYc3BPVmFHY3BVcmZlYzZSTXJSblgxSzdROWc0NlJ1T2pzdVZFbXEzaE4vSDNPb3d0WisvTzNodHVhaGg5QmludjhCekpGemowM2txY01sNlFqMThWdjhCcEZldDlBWjBoVEFQT29qbjBXSEY1OU9hcU1ieDVkWXV4UlRZSGs2bDFlUkFwMTV4aFJUdFRhTEUwQ1JmUmxKeXRTTFFoNVRCUk5ScDl0VGxtUUF0YldnTWJVZ1RoQzR2elJSMFpDMHZvRkZxVUpoVHF6bjR1dC9QUGRmTmlNLytKZjRweFdPS3V0allyMTBoSG5lTVp6NUN4eFJDd3RMcWEyaHFadU1lUjBiQkd3V0QxbUEwMHRhSFN5anRDR1F2bDE5VTdqWlNKVkZkYWhWWTIyaUxIcnk4c29RUStKUFZLV2NjTUx3aWNiRGFyZXY1b3FEdVYzcEE0ZGNycXJHelMxT2k5bW9rVXFneVhuVHNYRXhjOTdSVkVXS1dXenhvb0xLRzMwSUtrdXp0cVFsdXk0ZzhHVXpsWG16MUVTN3pRYWMzT2JjZ3Z2aHNQc3k0MExqYWtXOW9BU1RXSS9oRXRmTEhYdi9MZDBhMXFTazJvcVExYk56MlBGc1NQbTFNTWsycEdlckZCd2dHQXI2R0YvMXV2djN4MkpGVmVwU0xLYXlvR2h0TVYybktpdkZyVk4vVC9wNU1WMVJWRWVjMkdPRm9RZjAzSGFEcEdJOE82VlFNcVkrSTJXNElSUHdNRVZRRG9CYlFBQmdCT1lOZjFlaFhGbE40VFZhcXF5OThjNzFYWFZ4THFPblZvOHpjdmpmZFZObFFSbGZXYXFQamhBWjFUcjNmcUR2emhqM2ZYdVVuU1JkMHQ4NmdwZUdVT0d0ZHJRaER1b0tNS1Q5U1lORFhsK2dwSG9GcDlmWFMzMnFBbTFQb05ZOE12YWIySm55aUpYbHdXOGpTalg0di9ZVTdSMXBRRlZTNHZ0cVU5MHZsbUFQUjF0Q0MvVi9GSUd0NlFsVFFqK0RlVUxnQXFkNk9qTWJmNHRaamNUeXlGSlp6RkYwRU5QUFFDNlBTU1dSVUxWRmR5QldGOVE1VG1hQ29RRUxqaVNtV1hXZ2RiMUlHOC9vRzJ5aE16V1cxdVYyalNad3ZTZXNxZThIVEgzVEdiWlV0cms3ZDZVTmZKMHVIR09ucXd4VGIxdHdNQk9tSnNHMmZvTm93YmcwMU52UzZUSzdEOHVqZkx1K01CeWpIWTdFNDZVa0ZYdktQUnQ1MTE3T3pvT2VxbkxPWHhEVFlqN1hoRmlCb056bW5lMkEwWTNJVWw5QnArQ05RclN2ZVhFbDd2Wlovc09IaHd4L2FEQjdkM3hPTWRIWW1FNXVxVmJ6Mzc3TGV1WEkyZWZPU1IrKzkvNUpHVDh2MWtBTkJMK0lUY0w2UXhnNWZ3YWpreTg5aVgzYjBOa2ROeDlBNWZUdFVzMzRnWGE3d1pBUDBJUHlUZEo4ZUhjY25lMkRYS0FnR09JMXQybkVsMmRiZkVqZDZXeWZENDN0Zzk2WVlPdzh2dE83NXhEeWNrUFJhdm01OFo3YnIvYkFZVGZZQ2dvYkNFL2c0LzlFWGZZUGpWRjUyMUVDdGZHVDVPNzdVNFRVTWR3WDUyUEIzUDBDR3VKV1p5MnlZNmNuZjErSVBESGRzMUFoTm9hdTNoN1oyV2lDVmc5UWFhVFg3R016b1k3TmNUbGJsb1I5WU5XUEk2OUMvNEJGUklsUzV3MG1RaFNWM0hXM2trOGNDUSsrWUpSR2dhcWpqeFg1RjI2OWpZNHNzTktRUGxwa1QvQ3dIMGxQamw2QXNTTDRiQ0V2b0hmS0kwdWExaGtGUFhXVWxHdGRaeWZwT2V0cmFZMGgyaFRmMWhxOWZrSmxIa3Y3UlVxMGtZRDNUdjBnU3NBYU1uRTR2MjYzVkd4UFg5UUZQbDJwSkk3UFFWZTBGYllRbTlMdDk5Q3dDaVM2OTZ2Tit1K09JMHVqYjhvakp6cXFtOHI5dmJFL0tIcDRPSkwwWDhBNDJ0dW80bVQ3OFhOdzJ6dWQzK1VaUnFjVy9iTlJnSmJ4U2ZqMzl0NzFjdTlyRW1qbXJranU2eHVYYnY2dDdxbCsvZkxYa2FQaUY3V2hnTFZ0NUtWaWxVTHlyWndZajRLcnJRbVdyUkUvL3ZoOCtOOVhHcEI4K2VMODV4anNJU21zY1BnUm5jMENuelU2dzEvczVpSTRzZFFiSCszVlpSYWlCeVovdTBhMHBnaENZbTBKYmxjanVOTFhxVHo4SnQwMXFZSU84T09lSmxIWW0yVEt1ZHkyZzh3ejVuYjNzTllVajUydnNkTy9xdElXODFVZVB1ZG5tSFBHaWZxWWZ4Ump1OGRoOGozb2kwTy96MldrUFN6U2VLK3ZZV05zTU5tSU5hQUlvTkJGZ2x6YXdyc0pqZTFZYXdFdGN6elFhTExmbFhiYnBJQ3pJWkc4MStUODlPa09heTNzSVNmQis5ajFub0EwQ3pvSVRpOXg4SHZJK3FVUU1vQUFTZUl4MEw3MGNpeGRsckdGWGdYMHA4VXNYQmxwS3hVdStFazhrd0YrenNETDZ3NTlicDA3ZW42N2ZmbXB1N3RSMFEyQXZEY0t2MEgxWm1TOUlicVZlT3kvdTVjREw1UW1sMy9mVHQwNmR2QVlJTmhSMW9FNzRoeGFjUWh6WWdkWmY0eHl1S3ZaK2RMK0ttWVIrNmlkMmdsdkpqZUk2WEd5ZjU4MnZYZXE5ZDIzYzlmUDE2K0xxMHp3Y24wQTh4S2U5akJWYWdCRXBGcWRncDk0VUw3Z3ZucGQvekp6eVhMM2t1WC9aY3V1eTVJbk5Ddzkram0raHRiSWNVSEFBbHBPQ0pVc3hUNkNZbUplKzIyWGdicVZLUkZJVnVpamwwOVdkbnp2enMxTFBSWi91R2ZJUnZxRGlid0ROb1llVWJWRGFMRnFSZVdYZ2Q5NE9BcjBuNWFOZGRWTDNaWEY5dk51TitrNkcrcWFuZVlKTGl5WGNEYzlMZTlhNzVnSUZoRFBVTW8yRWFUUXhqYW1UazNKQUwzVVN6VWp3ZGJ5VnBkQlc1d21FQStCOEFBQUQvL3dFQUFQLy9MTzZoclFBQkFBQUFBZ3VGZHYyY0xWOFBQUFVBQXdQb0FBQUFBTmhkb0tFQUFBQUEzV1l2TnY0Ni90c0lid1BJQUFBQUF3QUNBQUFBQUFBQUFBRUFBQVBZL3U4QUFBaVkvanIrT2dodkFBRUFBQUFBQUFBQUFBQUFBQUFBQUFCSmVKd3N5cjhyUldFQXh2SHY4NXhCU2NUQXBkdDF1bTdxK25HUGRDTEpZREJSNmwza1Zjb2tmNGdzN0hhVGY4SjhMUmFEeFd6RXU5enVuWTdjenZSOTZ2bjRnWFA2NEp6TUowVHZVL3FhNkJtaXZvbStvZlE5VVJORWZSSDlRZlFkMFkrVTNxbTdSL1E4YTI3UTlDMW56aUNiWTBHZlZhVWhwYnNFOWVsNW5VSy85TlNocFNHYnpna01PTklQZ1lxUUhSQzhRbkJyYk1QWVh4RDBSRk9CaG5PTzljNlUzMmpvaGVuL3JXZVdmVVd1eElZU3AwcDBsRmhTWWxhSlJTVzI2cS9MaUlJUmg5cWxxNEtnZ2xWZE1xbEVXNG50dW0wR0JLaGV4emJSL2dNQUFQLy9BUUFBLy8vemJqeDNBQUFBQUFBQUxBQXNBRkFBaGdDMkFOUUE2Z0QrQVRBQlNBRlVBV1FCbGdHNEFlZ0NDZ0pNQW5RQ3VBTEtBdTREQ2dORUEySURtZ1BPQS93RUxnUmlCSVFFOEFVU0JSNEZPQVZVQllZRnFBWFVCZ2dHUEFaY0Jwd0d3Z2JrQndBSE9nZG1CNVlIckFnUUNEWUlUZ2g0Q0xZSTJna09DVTRKYUFtK0NkNEo2Z24yQ2hBS0tnbzZDazRLYUFwMENvb0tsZ3FzQ3NJSzBBQUJBQUFBU1FDTUFBd0FaZ0FIQUFFQUFBQUFBQUFBQUFBQUFBQUFCQUFEZUp5Y2xOMU9HMWNVaFQ4SDIyMVVOUmNWaXNnTk9wZHRsWXpkQ0tJRXJrd0ppbFdFVTQvVEg2bXFOSGpHUDJJOE0vSU1VS28rUUsvN0ZuMkxYUFU1K2hCVnI2dXp2QTAycWhTQkVMRE9uTDMzV1dldnRRK3d5YjlzVUtzL0JQNXEvbUM0eG5aenovQURIaldmR3Q3Z3VQRzM0ZnBLVElPNDhadmhKbDgyK29ZLzRuMzlEOE1mczFQLzJmQkR0dXBIaGovaGVYM1Q4S2Niam44TVAyS0g5d3RjZzVmOGJyakdGb1hoQjJ6eWsrRU5IbU0xYTNVZTB6YmM0RE8yRFRmWkJnWk1xVWlaa2pIR01XTEttSFBtSkpTRUpNeVpNaUloeHRHbFEwcWxyeG1Sa0dQOHYxOGpRaXJtUktvNG9jS1JFcElTVVRLeGlyOHFLK2V0VGh4cE5iZTlEaFVUSWs2VmNVWkVoaU5uVEU1R3dwbnFWRlFVN05HaVJjbFFmQXNxU2dKS3BxUUU1TXdaMDZMSEVjY01tRENseEhHa1NwNVpTTTZJaWtzaW5lOHN3bmRtU0VKR2Fhek95WWpGMDRsZm91d3V4emg2RklwZHJYeThWdUVwanUrVTdibmxpdjJLUUw5dWhkbjZ1VXMyRVJmcVo2cXVwTnE1bElJVDdmcHpPM3dyWExHSHUxZC8xcGw4dUVleC9sZXFmTXE1OUkrbFZDWW1HYzV0MFNHVWcwTDNCTWVCMWwxQ2RlUjd1Z3g0UTQ5M0RMVHUwS2RQaHhNR2RIbXQzQjU5SEYvVDQ0UkRaWFNGRjN0SGNzd0pQK0w0aHE1aWZPM0Urck5RTE9FWENuTjNLWTV6M1dOR29aNTc1b0h1bXVpR2QxZll6MUMrNW81U09VUE5rWTkwMGkvVG5FV016UldGR003VXk2VTNTdXRmYkk2WTZTNWUyNXQ5UHcwWE5udkxLYjRpMXd4N3R5NDRlZVVXakQ2a2FuRExNNWY2Q1lpSXlUbFZ4SkNjR1MwcXJzVDdMUkhucERnTzFiMDNtcEtLem5XT1ArZEtMa21ZaVVHWFRIWG1GUG9ibVc5QzR6NWM4NzJ6dHlSV3ZtZDZkbjJyKzV6aTFLc2JqZDZwZTh1OTBMcWNyQ2pRTWxYekZUY054VFV6N3llYXFWWCtvWEpMdlc0NXoraVRTUFZVTjdqOURqd25vTTBPdSt3ejBUbEQ3VnpZRzlIV085SG1GZnZxd1JtSm9rWnlkV0lWZGdsNHdTNjd2T0xGV3MwT2h4elFZLzhPSEJkWlBRNTRmV3RuWGFkbEZXZDEvaFNidHZnNm5sMnZYdDVicjgvdjRNc3ZORkUzTDJOZjJ2aHVYMWkxRy8rZkVESHpYTnpXNnAzY0U0TC9BQUFBLy84QkFBRC8vd2RiVERBQWVKeGlZR1lBZy8vbkdJd1lzQUFBQUFBQS8vOEJBQUQvL3k4QkFnTUFBQUE9Iik7Cn0KLmQyLTQxMDI4NzYzMzkgLnRleHQtYm9sZCB7Cglmb250LWZhbWlseTogImQyLTQxMDI4NzYzMzktZm9udC1ib2xkIjsKfQpAZm9udC1mYWNlIHsKCWZvbnQtZmFtaWx5OiBkMi00MTAyODc2MzM5LWZvbnQtYm9sZDsKCXNyYzogdXJsKCJkYXRhOmFwcGxpY2F0aW9uL2ZvbnQtd29mZjtiYXNlNjQsZDA5R1JnQUJBQUFBQUJiWUFBb0FBQUFBSWdBQUFndUZBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUJQVXk4eUFBQUE5QUFBQUdBQUFBQmdYeEhYcm1OdFlYQUFBQUZVQUFBQXpnQUFBVFlJUVFvMFoyeDVaZ0FBQWlRQUFBOXJBQUFWUENPV3cvcG9aV0ZrQUFBUmtBQUFBRFlBQUFBMkczOGUxR2hvWldFQUFCSElBQUFBSkFBQUFDUUtmd1lJYUcxMGVBQUFFZXdBQUFEdkFBQUJKSk03REZkc2IyTmhBQUFTM0FBQUFKUUFBQUNVeU1MT2NHMWhlSEFBQUJOd0FBQUFJQUFBQUNBQVlRRDNibUZ0WlFBQUU1QUFBQU1vQUFBSUtnandWa0Z3YjNOMEFBQVd1QUFBQUIwQUFBQWcvOUVBTWdBREFpb0N2QUFGQUFBQ2lnSllBQUFBU3dLS0FsZ0FBQUZlQURJQktRQUFBZ3NIQXdNRUF3SUNCR0FBQXZjQUFBQURBQUFBQUFBQUFBQkJSRUpQQUNBQUlQLy9BdTcvQmdBQUE5Z0JFU0FBQVo4QUFBQUFBZkFDbEFBQUFDQUFBM2ljaE0vSkxuTnhHTWZ4ejJuUDI3ZTBabXJtb0txdGVhNmhLRFlTZStuU1Vub0hMb3dkY1JYV3J1UXZUazVzUGV0UDhudStpT1JGS0l1akpxb1NzVmlpcG1IZGhrM2JXazZkYTd2em9PdkpzODhRU0ZYOVYrMWtxdU5lMTZQZWp3cGZTaW9xaGd3YmtaTlRWQWdmNFQyOGhkZndFbnJwL3Q4WDJYWGhVc2VadGl0NzloMDRkT1JZeTBtNmZldEdUbDdzbjRML2l2cjBLeWtiTUpoOU1Hck11QWtWazZaTW16RnJ6cndGaXhKTGxxMm9XbFd6cHE2aG1mVnRwWVhYZkFNQUFQLy9BUUFBLy8vS0JTTnFBQUI0bklSNGUyd2JWM2IrdVpjVXg2S29CeC9ESVNtK1J6TkRVaElsY2ppa3JCY2xpNlFlRnZXTUpUbld3eEVjVzdGc1dXdkxLenVXa3dYeThHOFRPdlpHWGtkSjlwZWtSb0p1MDJUaHdGdkFTZXN0dW0yeU5kWkEwaWFwaTZMclBEWncwMDNyYUZOaG00YzBMTzZRa21WdmdmNUJEa0hjdWZjNzUzemZkODRNRkVBZkFKN0U1MEFGaFZBS0JxQUJSTDFIejRtQ3dGSXhNUlpqR1ZWTVFIcXFEeHZrVjE0Vy9HcS9YeDF3TDdrZW5KaEE2WEY4YnUzQXJ2VGs1QjhtR2hya0Y5NThTejZOanJ3RmdMUGZBdUEybklGQzBBTVlLVkhnZVlIVmFGUkcwY2dLTEhXejdNblM0dkppdGM3NjdiVTNydjNFOXlzZjZtcHNETTJJa1lQeVl6aXpOdmY4OHdBQUtrZ0Q0RWFjQVQzWXdFdXdpV0d6bVRacEtGcTVhRmlWR0k1S0VaNWw5V0pZdWFZL1NoeG9xZmFGMnhLSDJ5ZVMwVkE0a2hvODN0ZzBpRE9PVkx4eXNGUmR2TDIxN1I0L09oVmdlYmM4TWxMSkFTQUlabGR3TFY2Q2NvQUNMODlMa1doVURKc1ppdWRacjBaRG04eGlPQnBqTkdoczRJbkJIYWNING5zOFBkWVlXOVZaT2RUaGkxdDZCblRkUHo1NDRObCswVHZPT01MajIvYk1WbGhIZHdOVzhIZmpER2h6bWMyajE3Q0NHSTRTM0FUdzVUMVA5L2VkMlYxdHJ4c01CZ2ZyN0RpVE9ETTcrM1Q3dkcrMHAyY25Cd1JmR2dCOWlUTlFwTlNIOXRBaXpkSWVPbzJXNU85dTNFQ2xPTFB3eUluekN4dHJQMWR5djJsdEdqMHZmLzNwcHppejhNekNHc0I2ekFtOEJLNy9MZVo4eUJJcmlYcU5CaDNjK2FNZHcyZUgyL2U2MDlhNlFQZnUwVjBtWG5mZ2x2ZDcrY0Fqbm5HemMzWnl6NnhXTzN0TS9zQVR6T0hBTzlZeGk3UW9pWHBXeityVGk1K2NPL2NKem56MzNkb2NLcE9YMXpIRHh6Z0RLbVd0UHIxSVNKQ1A1Vk9jZ1lMYy94NDZ2WWd3enF3dEw4RDZmZmgxbkNFeGlIclJhRFl6WWpRYU00cDZsb1FUWXltS0ZRVFdpV2s2L1NjUGFBMWF0VmF2M2ZmUzQxU2hTaTJOOVk5RjFPb3RGTTdJTit6TlRtZXpIWG5YNXI1MDkvYTVudi9tbStkZGZiM3VMOWZQSURVMDVzNWdSSjZYU0N3cWdUV2JhVHI5ektzdGFuVkpobHdLaW5GRy9xdXprUi9VMzF5YlE4bW5vZ3YxL3dZQVdNbjFRM2dKU3U5aW1NSUdJVWRoSmVsb2FPU3g3ZHNmRzhsOXQvWDB0TFgxOU9nR250MC8vZVBlM3ZQNzl6ODc4UERjNU9UTXpPVGtIT1Q1Vllzem9BUFRIZnhpNlEwOWZOeHhOSldhUy9aM0hHdHBUT0NNTU5yYlBWbnpHelF3SlFZZ2o2MEx2NFNYb0FoNEFHNHpGaTh2M0lHVU11ZDFvRUZEZVdqSXR1OHNPM1V5aDNoMmd1bjEwMXlKemRCdzRPZ1VRVGwxVkw1eGY3ZjVaMC9rNEQveTUrVmVKNldlTGl6T1l4L0VHU2dCWnJPeWlVRVE5RGxacDc5SUhrN0VwWE92bk96dnJtOXFxdS9HR1c2a3AyT01rYi83NGd1ME8xUmJ5NU1hc2RrVnJNVkxFRkN5SzhRVW9GS0VGNFFnL2lOaU0wd3VTOGpVOG5ENEhuYklGNndXSzNkNEd2bUdCeEoxczRIdDdoYUJyOTRhdUtjaFZUK2pxdzNlNytTOURwZkRVRkZTazZxSmprU3FBbVBXY3BmZDZkUjdMZmNrbzZOMWdNQUtnSTA0QXhTSmhKVThOS3UvZGdsOWV3bVhMU3lzTGVlNDJwbGR3YjJLbHhHTWtsN1VLeHBUZm1oUXo4T1BuNnVQeFJxZmVrUjMvbVUwTGkvdTd1N2VqUTdLRjE0K0R5ajdEUUFXY1FhS0FVVFZKcDZyZnZIMlQzcEttVkoxaWFVa2ZmN3ZjRVorVDlvYmplNlZVSzJpbnhJQWxSdG53SFBYZlp0MllGVTViNlpVcDQ0L1U2MHAwYWkxUm0zcVpFcHIxS3FwWXFyNjlOeWJMVnVLQzlTYTRpMU5PQ08vSys2TFJQYUpLQ1MvRzVxU3BIMWhGRnFiUXo0K1hWR1I1dVYvQnBUOUdnRFRPQU9sQUtLa0VwbjhVVEZSUlY5NSs0WDZFbHV4dXNSZTNQRGMyNStqbDg5elNaNVBjdWZsMGM4VkxnYXlLK2dEdEFwV1lBRVlMeUZmVENrYkpTaEZwUFVzd1JvajFxVDQ4aThTZlk4dVl0YnZhcW1RYXFickovWWUwNnBkN1Z1c25MR24wYVVianZlTWxIb0VDMzJmbzJMbXNQeVphR2NQTThaaGJhWER3aWpudFdaWHNCbGZBVlBlQlFXV1l2VWlUZDBsVHRaTEdnNUtldG9jYXQyUlJiVWo0VzBjcVdtY0dPR2pRMVYrazAvbmNVdjR5bXZkTmtmejk3cDNISThmUzNVL1h2MXJRNGxTOTRyc0NycUNWc0YydDgvZXRsbWlLR3Z5VUd2SDl4UEJkbnVTZFV2eGVLMGxhS3puaG5STlJ3Y0c1NXFjeklTanU3VWxUWmZ1ZHBmbmRDdGtWOUFxdmdKR2NLL25TdGxZSU9hMGthVjEwbjgxZXFoaEl1S3ZzMm9XajJuVnRoUzJDQVpqcFltTjF1aWVQTjUvdE5sdTZmNnp0YmFRalQxbXN2N2FVTkxXM3BrRXJHRC9CSzJDNWE0dW9TalZRMVJHc0t0RXhUS1FxLzN3dHJZRERlMWpOV29zWDllbVFsSTB4STgvZDBtbzhrWjF6WE1EL1hQeCtIVEN5QlZHUmM5T214UFYrNldhbkRZc0FHZ09YeVZYb3AvWVhab2xMVXgvNzdadEZYMXRya2haZWJGTlYrN2N1Uk9kUEZoUUxnMUZkSm9EQlFVZTNubEVmb1RNRE41c05hYlFLdFJBQTNRcG1lR2xDRWtFSVpPMEhnSWowbXplTEwyQ1VnZENMNU5Hbzlya2dNYThDM2w1WmNsWDllTjE3Y1p5dDhYbXJ4K1hxangvMFVzVlJrWmlEcGZCNis4YnZTK3gwT1VRQklkREVQemhGb0VUclI1ZGVkUDd0cnFxUnArNjJPY3FENWVwRFluS3hsNmZicnJJYTlyYVZhRXROUnNORFcxaWZ4QmREZmdGdjgvbkQ4aUxGVmFtVEtXeVdPMk9YRzVhU2JFVmppcDlsRm9YZ2w1QlNlbGJGeW43OW5CLzU2TERiZmRaOEpYWGRsb3JwOGZrYThnVDlWa1orUTNJWmlFR0FML0I3Mk1la2dCQVFRcWUyTmpiaWErQUx0ZGZ4WmhJL0plaVc4K28vLzlMUC92TEYyZmorSW84OC9ZMStWLy9wdjFCc2o2N2dnejRDdEUyczhuRENBbit2cnRoVVY5WVFHa01PazYzYXp0bTE2NHpCb1FPRmxDNWMxUU90S3I0a0Y0a2ZrQ3FlMGNrMU1hMWxXZzRGWkphalo2dVVOLzJSWWVicXlWZk5XaTV4VlZkNmZPRzFzT3JsZC9JWDliemhGYnplY3Fmc1RsUHg3UnFkM29qVVdnNTdxeStJMDg1dml2YytiLzd0RGwrS0pFNEZJL1BKQkl6OGVwZ3NEcFlYWjNYYXRQYzRNRFJwdmwwUzJzM2tXek9aenF3R2EyQ0Vad0F6RzEwQ3YxNGdhR050MjJHNEhSMEN2ZE9OVTVFM1kyMmdsNCtPbFFaTVBrdTQ1K0diT3dQait3NEZpKzM5djRJVld5WURQR0NEclNxN084R0tKQml5cmJySWhKam9sNjEyUXZRQXhyck5tL09FSnFKbzMyMllRYVhuK20ydUJSRGNMaERheU9vNHJZYjVQbUN6cUJWTU54Ung1eEtjeGt1NytacHU5WlNiQzJ6TjVuUThuQTRWRkR3c0ZydEQ4c2ZBd0k2dTRKZVJLc2dLUHk1M2FmNVhKL2UySXgwYVNlbVRacjNRL3Y0YmQ2NHkrTjBCRzNPQnQ4RE83WU91N2JaSXJhdFczbDNrMzlLeDd0R3JlV01VVzgyYW5VVlcvM0pJY0V5WWpJTEZtdEpFYnMxMkRhVzA1QSt1NEptOEJ5Wk5FanZsVmdwRmhPVndmaTJBY05vYjZKYi8rRDhQT3ZRV2JXTU1hYmJQM1Qxb09iUlI0LzhLc0JwMU5NYVhXNnZ4dXdLK2hvdEU1N2RvUUY5M25iL3FiOXowZW0yOCtiRlkwVXFWNWR1ZWd4RjVJOGt2ODJCT3VTeUpGY0ZpT2dOWmRGeXZwOXZhcEdYL3ZSY0MrbThoVVp0NitrTGFQbDNYRm9RMHR6djVMSjFuOFRMYURuZnoyL2Z0MmtITnYrc1JWSG5GcDZ1MVdnMWFxcTRNUFp3WFdFcHBhWUtxWnIvTi85YU5WVk1xYWtpcWdvdDMrUTZlTDZMdmFsY083aWJjdGs3Yk1yblM3SHZLT2VWQUtBVnRFem1HOUVvYkRxR1ltNmZVN0owNW9VcXJWbXIzbUxZNGwwNisrd0x0VHBHcHk0MEZRb0kzK3FqSzJtNmt1N0wvbjZBcnFMcFN2TUEyVmVYYlVacmFKbW83RFlQWXJFN1VsR0NqNWs5cFRiS3NJWHphYW0vUHRkZVpOQ3F0K2dMRzArL3h0VDEvbEtqbmtVRkZRNGIrdTJIM2hUSHRyTWZ5a1hOT3dLNUdwR2g4T2RvV1hrR2tveGtJRk9KTlAvZW0yajJ2ZXU5S0hpa1IvNkhJNlJYV0xNcitJZks3QnVCT0lEUlJFd3BKM0pqVHYyeERROGdZcVhNWkVGTXpQMmllRjdRYUFSUy9wank4NU1pTFZ0clpxeGVmZnQ5b2ZaYUkxM1pGKzBhOGpWNzdXMFZWbDczdUVIaVhWdXRyRzlId0w4M0U2MzArN21rdzJoRnR3dytFeDMwTUhaaDdUTnhSeml4dzhVbVhUWHBtcjVFb0UxaTNFMDJkMCt3WVVZc285WHpXN3dXRi91M1hORG1TbFRvZVdVTzRMSXI2QXQ4Q29yeW5wWFRFVzBpZnBWN3hzdzl3cHJSbHIwblQrNGxINnVQWVh4V2k4OWk4ZWxldlhEaGxWY3VYSGoxTURjK1BEenE5WTRPRDQ4cno4UXBBUFF2K0lUU0c4aElJWkhBOVNLZGVtSSswdUU5TUQrUER1M1MyazFycS9PNW5Ec0IwR2Y0Rk5qSittYWN5MkorYnNzbEx4b1ZSWnJyUDVrSytiMHhTMS9OWkNJK0xqV01SaXlONWgvY2t6NzVRSFZOU0xEMWhzWHdyaWJwMEtHb3FtQ0I3R3ZPcnFDUDhDbnczKzBackxSdXlPdlQ0ZTBYQS8rVlBzZ21IQ2xmVFoyOUt6blU0dU85TVdkWDFXVDk1UEdZR0d0dm5kYUZmV1AyQ3FIQzdqZFAxZkFlem1tN2w2L2NOUmhLbWRWbDZlYUd3Y3JjakdVRVFGL2pFMUJJMUc0VUNSOEk1WTJTUnpLU1hMRDBTNDhYSUxYT1ZoS1cvL1B6bjNkMm9pMzdYUDFPVzdSY25sbTZIejBrbjU1ZElqRXcyUlgwTVQ1QnBxZzdZbEN3R3owMFMyMWs2Yjk3RHZEYkhBbGZxTDZ1eXM0NXRoblExTDhYZWZqWXJyclcvYm9JTjJiandxSGFjSWtoZ0ZvWDVrc0R3NG5Vbm9pQzFaOWRRZitoOE1BSGdMejVwelVwd3F2KytJMEd0Y0ZvWkxTS0ptMmR4MU5UNDJ5YVNYWWViWXVQT3RObE1UdGJ6NnFzblk2QjZmb0p4RG04MjdlR291R0EvRzdyazRmbWx6cXJYU09HY202NHk4MU8zTDl0SXFMVXZ3b0EzY1FuRkU5cnhqR1A1S0ZMVk5TTEdtK3FVZjR0ZWl2V3hwV3A5Ly8wdWNHRmU5dStmK0xzbVBKK2g4ejdueXVjRVNDeTBYRnZUMm1iUzZxNmV5NGppaVBkaklydnFZOVhjN1dSMFliaC9XRlBzS1h1ZnJ2Z3IzQUVHblZjcmJmUlI5dnJkVlc5WW4yWFJXM3ZDRWQ3QXhPOXdYYXoydG9URC9jRjBVUFZ0VngxQlNkVXlSOEtQanZuMEJzbFI2QkdlVC9TakFCZUo3Mk9FYUpSd2V0bE40bXIyMWxYajdBYXM5RW9INDZNL3JMSDFNcFYrdmhnVit2QU1RQXljelZsVitCTDlIc3NRRHNBU29DR1hBRkJKVnhGSGhRQ0ZVQk1FdW5LUDF5ZG1pTC9kMlRUeUljL0l2bGpjbWxnbFBpWWEvRmtNajRhQzRkamwvYmRlUFRSRy92NCs2NVA3NzgrQ1FocXMybFVscjlIVU5ST3VFV2JOSm5SdW5DNGJqU2VURjdpSjYvdm43NStINi9jQ3dpS3M3dFJGTDlEem1lTW9xcjQ2dTZyTDZuMnJqNUhNSGhoSE4zQ1VTZ2kyRmhKbEhKTjhoOHZYanh3OGVMNDVhbkxsNmN1azNXMU1Jcyt4cXl5VG9nSk1TYkdVQXdsSE85ODZxbk9NMmVVNzltdU0yZTd6cDd0T251bTY2eVNEeTljUUxmUVY1aUhEcGdERFhSQUJuSm5UcUZidUpaNE5jZEpIRTFSTk1PZ1czSUVYYnQ0NnRURnFUZjJ2bkc4dmtaZFU1K2JQK0FEdEx6K25xaDFFUzNMWllDeXIrT3RNSWpmSjNqMG00ckVCWU1jRnd6aXJRR1dEWkFQT1UrcEM3eE8xaktiMXA3blJaSG5SVkVuQ1Q1SjhnbVNnZzJaMFMzMEVEblBLSGxvTDdxR3pLUmE4RDhBQUFELy93RUFBUC8vT1ZXTFB3QUFBUUFBQUFJTGhYbUgyTk5mRHp6MUFBRUQ2QUFBQUFEWVhhQ0VBQUFBQU4xbUx6YitOLzdFQ0cwRDhRQUJBQU1BQWdBQUFBQUFBQUFCQUFBRDJQN3ZBQUFJbVA0My9qY0liUUFCQUFBQUFBQUFBQUFBQUFBQUFBQUFTWGljTE1xeFNzTlFHRWZ4OC8wRFJUR3RWNnkxWFRyRXFOamVobTRLTnNOZGlvSVhCQlhxNE9oVENQb0c3c1hOV1JkWFg4REJ6WmZSTHBHVVRvY0RQMzF3eVJlb3JCYTZJMnBLb1FlaWNxSlNvaDRwOUVxMEVWRU5vaFpFellsNm85RDFxdmRFZWZiazZXbk91WHoxbDR4b3FWbjlhcE9CU29MOWtLdmtVQTF5dTZHckRwbE9DZGJtV0RuQitvVGtscUFKUWNPbERiVzNaNEo5c21OUGJPbUVpVFpJa3pXNkVpMnRrOW8zQjNxaEo4ZStIRk01K25KMDVOaVdZMWVPSXptOEhBUHpGT1lwN1lLaHpUaXpHV083b2lsSEpzZDQxY3phQktqZWExdi9Qd0FBQVAvL0FRQUEvLy96amlnTEFBQUFBQ3dBTEFCUUFJUUFzQURVQU9vQS9nRXVBVVFCVUFGZ0FaSUJ0QUhnQWdJQ1BnSmtBcVFDdGdMVUF2QURLZ05JQTRBRHNnUGVCQkFFUkFScUJOSUU5QVVBQlJnRk5BVm1CWWdGdEFYa0JoZ0dPQVowQnBvR3ZBYllCeEFIUEFkc0I0QUg0Z2dPQ0NZSVVnaVFDTFFJNWdrbUNVQUpqZ211Q2JvSnhnbmdDZm9LQ0FvY0NqWUtRZ3BZQ21RS2VncVFDcDRBQVFBQUFFa0FrQUFNQUdNQUJ3QUJBQUFBQUFBQUFBQUFBQUFBQUFRQUEzaWNuSlRQYmh0VkZNWi9UbXpUQ3NFQ1JWVzZpZTZDUlpIbzJGUkoxVFlyaDlTS1JSUUhqd3RDUWtnVHovaVBNcDRaZVNZTzRRbFk4eGE4UlZjOEJNK0JXS1A1Zk96WUJkRW1pcEo4ZCs3NTg1MXp2bk9CSGY1bW0wcjFJZkJIUFRGY1lhOStibmlMQi9VVHc5dTA2MXVHcXp5cC9XbTRSbGliRzY3emVhMW4rQ1BlVm44ei9JRDk2aytHSDdKYmJSdittR2ZWSGNPZmJEditNdndwKzd4ZDRBcTg0RmZERlhiSkRHK3h3NCtHdDNtRXhheFVlVVRUY0kzUDJETmNady9vTTZFZ1prTENDTWVRQ1NPdW1CR1I0eE14WThLUWlCQkhoeFl4aGI0bUJFS08wWDkrRGZBcG1CRW80cGdDUjR4UFRFRE8yQ0wrSXErVWMyVWM2alN6dXhZRll3SXU1SEZKUUlJalpVUktRc1NsNGhRVVpMeWlRWU9jZ2ZobUZPUjQ1RXlJOFVpWk1hSkJsemFuOUJrekljZlJWcVNTbVUvS2tJSnJBdVYzWmxGMlprQkVRbTZzcmtnSXhkT0pYeVR2RHFjNHVtU3lYWTk4dWhIaFN4emZ5YnZrbHNyMkt6ejl1alZtbTNtWGJBTG02bWVzcnNTNnVkWUV4N290ODdiNFZyamdGZTVlL2Rsazh2NGVoZnBmS1BJRlY1cC9xRWtsWXBMZzNDNHRmQ25JZDQ5eEhPbmN3VmRIdnFkRG54TzZ2S0d2YzRzZVBWcWMwYWZEYS9sMjZlSDRtaTVuSE11akk3eTRhMHN4Wi95QTR4czZzaWxqUjlhZnhjUWlmaVl6ZGVmaU9GTWRVekwxdkdUdXFkWklGZDU5d3VVT3BSdnF5T1V6MEI2VmxrN3pTN1JuQVNOVFJTYUdVL1Z5cVkzYytoZWFJcWFxcFp6dDdYMjVEWFBidmVVVzM1QnFoMHUxTGppVmsxc3dldDlVdlhjMGM2MGZqNENRbEF0WkRFaVowcURnUnJ6UENiZ2l4bkdzN3Axb1N3cGFLNTh5ejQxVUVqRVZndzZKNHN6STlEY3czZmpHZmJDaGUyZHZTU2ova3VubHFxcjdaSEhxMWUyTTNxaDd5enZmdWh5dFRhQmhVMDNYMURRUTE4UzBIMm1uMXZuNzhzMzF1cVU4NVlpVW1QQmZMOEF6UEpyc2M4QWhZMlVZNkdadXIwTlRMMFNUbHh5cStrc2lXUTJsNThnaUhPRHhuQU1PZU1uemQvcTRaT0tNaTF0eFdjL2Q0cGdqdWh4K1VCVUwreTVIdkY1OSsvK3N2NHRwVTdVNG5xNU9MKzQ5eFNkM1VPc1gyclBiOTdLbmlaV1RtRnUwMjYwNEkyQmFjbkc3NnpXNXgzai9BQUFBLy84QkFBRC8vL1MzVDFGNG5HSmdaZ0NELytjWWpCaXdBQUFBQUFELy93RUFBUC8vTHdFQ0F3QUFBQT09Iik7Cn0KLmQyLTQxMDI4NzYzMzkgLnRleHQtaXRhbGljIHsKCWZvbnQtZmFtaWx5OiAiZDItNDEwMjg3NjMzOS1mb250LWl0YWxpYyI7Cn0KQGZvbnQtZmFjZSB7Cglmb250LWZhbWlseTogZDItNDEwMjg3NjMzOS1mb250LWl0YWxpYzsKCXNyYzogdXJsKCJkYXRhOmFwcGxpY2F0aW9uL2ZvbnQtd29mZjtiYXNlNjQsZDA5R1JnQUJBQUFBQUJkUUFBb0FBQUFBSTBBQUFSaFJBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUJQVXk4eUFBQUE5QUFBQUdBQUFBQmdXMVNWZUdOdFlYQUFBQUZVQUFBQXpnQUFBVFlJUVFvMFoyeDVaZ0FBQWlRQUFBL2NBQUFXZEorTWpFbG9aV0ZrQUFBU0FBQUFBRFlBQUFBMkc3VXIybWhvWldFQUFCSTRBQUFBSkFBQUFDUUxlQWp0YUcxMGVBQUFFbHdBQUFENEFBQUJKSVhoQ1JCc2IyTmhBQUFUVkFBQUFKUUFBQUNVMVFiYS9HMWhlSEFBQUJQb0FBQUFJQUFBQUNBQVlRRDJibUZ0WlFBQUZBZ0FBQU1tQUFBSU1nbnRWek53YjNOMEFBQVhNQUFBQUNBQUFBQWcvOFlBTWdBREFlRUJrQUFGQUFBQ2lnSlkvL0VBU3dLS0FsZ0FSQUZlQURJQkl3QUFBZ3NGQXdNRUF3a0NCQ0FBQUhjQUFBQURBQUFBQUFBQUFBQkJSRUpQQUFFQUlQLy9BdTcvQmdBQUE5Z0JFU0FBQVpNQUFBQUFBZVlDbEFBQUFDQUFBM2ljaE0vSkxuTnhHTWZ4ejJuUDI3ZTBabXJtb0txdGVhNmhLRFlTZStuU1Vub0hMb3dkY1JYV3J1UXZUazVzUGV0UDhudStpT1JGS0l1akpxb1NzVmlpcG1IZGhrM2JXazZkYTd2em9PdkpzODhRU0ZYOVYrMWtxdU5lMTZQZWp3cGZTaW9xaGd3YmtaTlRWQWdmNFQyOGhkZndFbnJwL3Q4WDJYWGhVc2VadGl0NzloMDRkT1JZeTBtNmZldEdUbDdzbjRML2l2cjBLeWtiTUpoOU1Hck11QWtWazZaTW16RnJ6cndGaXhKTGxxMm9XbFd6cHE2aG1mVnRwWVhYZkFNQUFQLy9BUUFBLy8vS0JTTnFBQUI0bkh4NEMzQWIxM1gydlhlWFdEN0FCN0FBbG9BSWdNQUN1M2dzQUJJTFlBR0NBQW1DQkVFUTRQc2hTM3pwTFVxVVRNdW1MRWVTRTF2ejI3SVN5N0NIanVkMzFOaU4zVFFaVDJPUG5NdzBqdTFNN1RSbTVDclRkSnpHaVd1MzhZTk9wS2EyT2F6em1IRFIyUVZJZ3VvME14cG9SOXA3ei9tK2U3N3ZuTHVnQXRnQVFDZlJNc0JBRmFnSGFxQUZnQ2N0R01ZTEFrMWhQTXZTQkNHd0pFblk3b2NyOXorSkovZDg1SGo2ajV3Wjc3M3YyLzMvT2ZjY1d0NVlnRithL3VJWHhiMFhEeDJhdkhsVGRNRi92UWtBQUtqd0pnRHc1eWdQcW9BS0FKTGdXWVpoYVlVQ1FwNmtXWnI0b08zMWFyd2F4dzI4K0UvdzRKN3NzUG8zOC9DZXhjWEFzWERraURpTThodUwxNjhEZ0FFYUFOU004a0FGRE5JelQvSituVmFqVUJDRVR2NmJ4bmgvS0JoZzZPMEgrc0x6c3d2dXBBM3lxZDV6QTIwek0zdDZNbnVQbjVvNW1ldTdDK1V6dlZ3M1Y0a3JFK0crYVE2ZTdoVTgvbzBiUFZsL1RNb2Jna2hoSFhuUUZXQUdvTUxLTU1GQUhQRitIVVV3REcydFExcU5Uc2Y3UXdLbFVFQnIvOUZReTU3ejJmQndZNGdNTVcyelhUWnJKdXBJTnRPMmFXWHl6RUJ1K2U1ZXdlVnNabU1IejdSSHA0UE51L3htajhTTmpDa2tjME9XSWFKWjNoL2FSUENGaHg0ZWYrcjJpWW54YzhrakIwSW8vK0E5ZDMvM1VPZllWL2ROenhmemxQWm9RSGxRSTU4WllTRjRnaVlzQkgwQkhxc1ZQM0I5VnZjSkQ1azZsRS84dk92enJyTDNxOHJleDBwdmV6NnIvYlFkNVJNZmRvbS8yT1FnaXE0QXE4ekIvMEdCUUFzOHBsQkE3dlQ1bHIzM0RVZUg5UUlwT09MN2UyeDB0c01XSWUwWGEzOGFzYzBvSHowenNIeDNhb3VJdHBsUVk4UDNPc1VQVGZZdEhQT2JPSGpNUXZJWVRWb3crc0pBR0RyQ3VRc0RIZUxiY1pRWGIwTHR4aUlNaXl2Rk5XQWQ1UUZXWEVOZkdMZ2dGY3ptZnZBeGxBY1ZwZjBJK3NMQWFhaXBSZm1OcTVzOHZJenlRQy8vUDBueGdod3hGQkpvQXFNeHFVWUpqTDR3SGRIaHFkZW5ML1JucXd4S2ZQQWZ1SmdPVjlSVlpsQmUvUHJGaTNEL3hpSTh4UjF6UHlZK0M2Y2U0K1k1OFhKcDc4TW9YenBWa3VKRElYbjNyVjBIdnVyQ0ZYWFZQZjBYY3N0dVhGRmZuVUo1Y2VxaDFoTThuTnBZaE04OHpCL3ppMC9KTmRKZVdFY3o2QXBvQU0zbHA2RFRhdW9RNjQ4anFVNktwd0hOSjVlOEUwdXB6S0dBZCtLdVpIQXliczBNU0w5OXlpZk85ZWVYZXJyUGp2WS91dFNUYk4rL0ZObTNGTjIvMURaM2Vxc09QVEwzbXZJNnBERnlXMHJmbnpxVnVXOXNQcENZUFhRc216NkU4cG1Kb1NPdDRoOWc3OUJnaEFlbFhHZmtYT3NCQjRCOVIzS1VsV0daSGNrVG13cUNkRm1xdngrOVJ6ODZXWUxoSGpqUnBSb1AxTytxcmJBRW93ZE90eFd6M24vNkY5a2g4cEhqSlV6cHI1enFVN2pkT0JhcktXRmhVUjdVQXQwMkZvS2tzUjFvdmp0MXg4blJPMGNYVGduZEIyWU85cWZuVUQ0MXV2ZWtTdndBNnNRYmNId2tGZklWOWFVc3JFTVJYUUV1QUNRTWdweDFNTUN3TXB4UWFFc1JDb1VFa3lxNjBjZkpSVWZFT0M2MEQzdnNXVmMwT0JXTnpwbDVmY3ByRHhwYmJWbGZJSHBZMmRibWR2dTd3emEvem12b0Uvd2ovb0REYTNLYVczWXhQcDJucVZkbzJ4c0FFRXdEZ0lJb0R3Z0pEUzFZQ0JyNzV0S3J0ZkROMmg4dW9Wd3l1ZkZpTWMvQndycGNjN3BTbGNnVko2VWtrVXpLQ2pVZFBLN0FNd1A5VlowOTRUM2E0ZXhJMC8zSytjTmFueDR1aWc5NXJLbmMxSEg0bUhqODhqM1NmaU1Bb0lUTUkrQXhudFRwU2xVTUg0NE83cXFveEhCOTBQQzlNZkhiS0M4dUIwK0VncmNINElJc1BRREJMQUFZaC9MQVV0U2VRa0VVMTVJNmpiYTBDNDBGUW9Lc2h0bXFvV29NdzNHcVJmZHNieFhFTlc3TjVaeTRkb0JBRUsrenFGNUVlZkdKd0VJd3VCQ0E4K0lUZ2VPaDBQRUFuTjlZaEkvYUJsazJ5NHEzeXpGSEFVQW1sQWYxVWt5TXAwb0pDenpFdnN3ZkgvSlUxVmRoRk5kNGJsVDhyd0FFTVArS3JZZXg5OXBlRmhjTGN1MnloWFg0QjdnR05GSUZVZHR1eHdzOFJrdVpzcExYYlZuZmk1MVpMalBEc3pFVlRzYjNkVlRpOUc0MU0yamp0UDRtV3pKb2JsWHVIVS9kTThVN0xESFJrTGI3T3IyK1h6SldWOSswdnlOVzFJcTVzQTQvUlN0QUszVmZxYkpvZ2laNVFtSktydGt5ZGNzOTdnWWJVMkdhanNzNVZvZHNZeDQ1Zk5DV0RKcGFuTlpoMnF2aGxRNUxESzI4T21kMDc1bVFRbmU2K3FiNWVNeGwvNWl4QWdqc2hYVjRGYTZCcGgzb3RpdTMxTTNlSGp6STVmWUZ1WGFkaDJTTUxST2hTRnR6U0djMTVKU0hwN3Z2SFBkWjlTMlV0bnN4MlpVeXFQd2FPOWprRHJGbFdMYTUrOHZrdGFteEJpYVhMN0UzWUwrVlBiWjU5dFdOOEszMElSbkxEK0VhTUFCN2VUeFo2UmJGVm1mR2VMbU5TZ2cvbkpqMzlFKzFDQW1Uc2tMOFVWVnowbVdNVUNiajhQOHZJRXp0cElNenltUDdlaFpIT08rUXY0bXY2eGl5NjFXODFnenROWTIxVGEzbWNRQ0JHd0Q0TUhvTFVMSU9PMUM1OGdtNWlickhPMm9TRGZVRE1ZTkx2YXQ2bDhyaXJGVHRWeDRZaDkrS1ZBeG5SbXRyQktMYTd4Nk5pN3Nsem1EQkJ0ZmdHakFEYjdtekNJSkNRZStzUG9VQzI4SGVjNjBUdEsycHh4SFAxT21aTVY5c3lOMDMxY3JFVlJqWmNaaThNMElQVzkyNjFpWTZ3WnQ4LzhZWWc1UTEyM21VNFNiR2szZmQ1cGZxRVpzOURDMXUxejh6Vm1kcWQwczBXdFNzR1FENE5sb3A5Y1B0T2lUa3BoZ01TREF4OCtWY1N3UHVIT0hpd2NwNHRoM0gwMDFwYnc5YXVSbWpmWW13MlNaZWc1eW1zYmJmNVJXL1ZTaEllNEkvb2F1SUFUMEFBQVZJcGJkai9RNnRBR1hSSDZSNEpNMFNoUGx5Ymc3OWNmZHJTd1BUaXdhMElob2hmRlA4NkhlbnpnSUl1TUk2K0JOYUFXcUpyV0NnYUdsYVRlbW9UeVFVWjNQbklWUmhDZ0pXNjVRZEtqMDZ2dkVvVVlXcElZcmkrRlpjZEFPdVNWNHV4U3hDcEVwQUZUdVFsb1BlMTBIZ3pDalQxbHJoMjIyUGhYQThub3ZoZUs4MnpmVklIS1IwYVhjUFhPMnp0UW9PamsrRVZTWk5PUS9iVDlzOHd6WFFXSjdEclRSTEVaMGozaDBzeXhGdUpYbExmL0FkdUFicWdiRmNEMFVUS1k2U1JaRy9OVGpEWldiOGc3TmMvNHpMTTh5SC9OS1A4dWplbmp2SHZjWGZ6cTdGN3E3ZTVHSjNWMHFlM3o4djhQQlR1RmJVTmxHV2NSMmlaZGNpeUIwK1ZYMnBRNEhaeDcyeXhQMU1PNG5VNXI4cDk2bnI2TVZPczZja2NQUFJweUFzR1JYekc3dGxFdzh2ZTdFY3MwS1FET1FXVGV4VUJMUllUTWkrMjF2dXlaZWVLamVVNjAvZHpmaTJMSGtqQitGT1F5NmV5em00QmhyS3pvVWltTTN6cU1HTldZOWV1NnZCWU11YVkzQjFtb3RWZFZkMlJNWHJBQmIrWEZpSDUrRWFZRytkRVc0ZEVhUUpvVGdnUE5NNnJXK2hPaGxYekJuMlJyZyt6cHRwOHBLOGhXa05OY2NETFNQS2dJTXhPN3kwZ1RVYjRrNTN3bTR6T1RRR2o5bkVxSzN0bktmYkx1WGNYbGlIdTlIQ2xxZUhCTW1aZU5tTnlqejkrNTBCSEVaNmE3SzJ4SzZ6eXZNUnJNbGFaNmhSTmZpVUhaNTZReTFVUnlvZWVDQXUzbENyVGFicUNvR29sL1lPRjliaEozQlY4Z05xZTU0b0tZNHMyZnB6VzJwSUczdTVucXpVQ0IxanlpNUJaU1poU0h5TDFFdGxDbmVMaGd6TkZ6VVlCUUQrR3E3Kzc3bmkvdDZzRFZmZ3VNcEdQcElUTitDcStESGRUOXY2YkZBdkdvcHJVd0NnTitDcVBGZVVyOTErd21pc2VMOGtzSGs2MndBaHhPdDNOWHlwWDRXa2FjTFE4TVgwdTdOMThyOGE2MC9EVmZGOWE3ZlYybTJGcHJJbkE2eW0welpibWhZL0I3RHdGZ0R3WDRvODBDUmJObGNRRkYyNnl4SUU5NnU5QTY3S09nS3ZiNjRmSDEwNU1NaFZxcXJ4QmlzNUE5R0hDenBXcTNGcUYvNzdzMU02cjA3SFVYY0NBQXV2Rlh6d0E3Z0tEQUFRY3MzSTVyK0RrVHFrcUc2dTA2dlY5b1JlUFpwbHBLbExaVmQvSlN1K3I0K21mMFlRa2FxWW40WWZpNTlZY2pTZHRVTFZ4bWUrSENkelZmZ2NBUGdkdUNyZDgyZ0JTb01qNUlucVNwaDh0eGJHS3NWWFJDVUh6OFU5NHYrTFMrOWpvTDZ3anJyUkZWQURXa0VNQUZKRGJhb2FrNDgvSkd6V3NuenIxa20rS2ZEU0UwVXdyRUxCRnN1TmtaNC9xbUU2MkVhRHU2MDFzOWVYZGF0YkJyeWhVR2pNcTAzWW5URkR5cHh0RVh6TnNiaHI3cEdJbCs1cFpyc29id0wrQjhOUjNqQk5XZm8zZmptYUNnM0ZEZEZRZUxJMTV1VnpjV1BnZ01OOXNLM3JiSUJUeFRWQksvdU1OOXlrZHg4VnpIMlNYN2dLNi9DbjZCSlFTWlZCV1ptL2NFbGE4ZmZhWFgxelFYL0s1dXliYldXVEFTUG5sWCtWNFFQeDIvNzZYRy9iZ2ZpZXA4K21ZdDEzWE94T1R2YmNjYkc3YXhKQTZaemdsOUM5OHYxVWtDYnhrTUJqUEdHby9mTGNIZFhqUXZTdSs1V2Q4RDIvMHJyeFd1Y20vejlDbDZSMXRCREhTZ2JKYnBrbllTR3FLK2N1ei9qNFlIUEN5bktUTFNPN1hTUG5ScUZHNlIwK3UvODJMOWR1TWJjd3p0dTZnek56aStrdWFjL2ZGOWJoVDlBbDRMakZaMmhoeStVSmRyT2JhWXRHODRQRUlSTlBaVnE3SjhjT0tRZjNzbjdlbURTeW85TkRrLzJaWURRMnIweDRITlpBZjRUdmFuUEdUSzVRRThWM0RIWEZwclM0S3UyUDNkWXE4U3Nad25WMEw2aVc3b2NXV3JBSVVNSk8yM2toVkp6aVlYK2FGbjliQldmR2hrYVZvMkxoSHhtRm1zQTFEczBMQWZpa3VCaVB2MkpNV0pvQ2pVVWRBNmxmb1h1bEcrMDJqaTBBcElXZ2lhMUtlemt4WS9UckVtRlhtdXNJbUxsbXl4QjAxLzQyb0hMcDA3UEprOG9PajlNU2NPWDRlSHVEeWdBOVhTOVhLc2RIczdmSDVEN0NGOWJoVFhTcGVCOFZOT1ZSRkJwcXgxY2RxVUZ1QnowYjlUUHROTy9YRDlyZ2ZHakk3Ums2MFJuczBRU3M3ZjdKampyTG1LVjNYSmk5MWpQdXl6aUVoTlZYUTcwZjN0ZXgvNW03dTFxYm5XM0pzMk9NYmZkQS9KaFVCMkFlQUZTQjdwVm5uVGdTTElLRnFFUEVHV1BtOW1IeFdoMjhYSDNnZEpJNjg4YmZEWFg1cDEvKzhRbFppNWJDT3Z3VnVnVE13QTBpbXhVZENnbkJyWDVTUEdFVGtwSWxOMEZwTlRxc2lGQVNvVmFqZTg4N0dYWjNzMDJtd0tUZjJlZE5DUnBIVS9zK283MDk3T0pTY1pPdDArRk1zdjZ1UHFXdEw5eWFDYXJ3cGlncjVGek5DWC9uaEJtdmRZYXRiYU1lZUtneDQvY0Zva0YvVkh6SkdIYlllYWUycVQ4c0ZMK1RGWDVkbUFKNXNDRDFUYW02QmJsUmwzalUxUGdpQk5LWmFLT2hhYyt6WG5XN3phRFRzelpUbjNSUExCUUFWVmdISy9CZHhJSmUwQW52QUFyUVcvcDI4eDZzaG5xQUFTQVpMYTE4cC9hOXpUazFVUmlDaytnZHFTYXBFaTJVUXY0WVJYMmgwU0ljelhpT0xWUnA2bDdvZkdaazZTYy9tTlkvSVA3NzE3Mkg1eGdwMTdjS1ErQkdhUzBiVXN0RUNVV1pRTSt4NDFYcWVyKzB4UXVHQjZEbHIzeUhaeG15OHhzalM5ZGVrdGIrZldFT2ZnUDlXTXFKZ0R4TXc2dGhNZmMwZHZqUFR4YnphZ0ZINERYa2t2eEJDTkpCUHNocmVTMnRmZTg3ejdjLy84S1JOeUt2dng1NVE4NGZQQWhmUW5YeWU2ekFDcFJBRVJUQnpqdVdseDNMeTg3SEgzY3VQK2o4MnRjYzhoL25GWm1uRnZBNnZBWi9oaGlRQnNlQUFxVEI0M0pNTDNnUVhrTjFrc2ZiN1VHN2xpQzBGQVd2aVFmaGxUZlBuMy96d1crMi9XMWlxQVZ2SFN6T04rQTZYTjM4Vm1iZWw5c1BWK1hHQ2tFdjZnZFgwVlVwSjdMcytNNlFKcHJTR0duVVQrbjBsa2FkdmhsQStjd3VnZ1hwWGFyczNaUk96emJwR3UzS0pwMkJNK3Iwbk13SjlNSnI4SWdVa3d4YXRDM3dTZWlOUkFBQS93TUFBUC8vQVFBQS8vK1N6TCtnQUFFQUFBQUJHRkdIQUk2UFh3ODg5UUFCQStnQUFBQUEyRjJnekFBQUFBRGRaaTgzL3IzKzNRZ2RBOGtBQWdBREFBSUFBQUFBQUFBQUFRQUFBOWorN3dBQUNFRCt2ZjI4Q0IwRDZBREMvOUVBQUFBQUFBQUFBQUFBQUVsNG5DeU9zVXFDWVJoR3ovTTZWaFEwV0M0ZjhhVkNSZ2d0U1M0TzZSSVVGVzNSV2dSTkxkMUdZL2ZRMU5Kb3RRUlJrMHZROEhrQlFVajhSb2h2L09KMDRPRTVjT3lLVlY1QVkzKzFEdEVxTkd5ZnFDRlJIMFRicFdHWFJFWkV2UlB0a1dnWFJMdW1hWlVwYTBUOU1hc3hKM2JHbnI0NXNCOU85Y2FoTFZPMUpZTHVLVnVScXZxVVZhSm1pOGhtQ0h3UjlFbmdsL1ZDSU5nY3dRcFVyZWhaN3VxWW9Cc2ZhWWVtTGJDbEx0djJURnQzM2xQWG4zVHJtYldaVjJKTmlaS1NaMG8rVkpwc0tMR3B4TGtTS3d5OHo0Q2k4dDVJUzJYdjZjZ2ZsS2dyMFpweUkyOENPcE52b3Y0UEFBRC8vd0VBQVAvL2hPTkpPQUFBQUM0QUxnQlNBSW9BdkFEZUFQWUJEQUZDQVZ3QmFnRjZBYWdCemdJQUFpUUNaZ0tPQXM0QzRnTUtBeWdEWWdPQ0E3b0Q4Z1FnQkZnRWtnUzZCUUlGTEFVNEJWSUZkQVcyQmVBR0RnWklCb0lHb0FiY0J3b0hOZ2RVQjQ0SHVnZnFDQUlJWUFpU0NLb0kxQWtRQ1RnSmJBbXVDY2dLSGdvOENrZ0tWZ3AwQ3BJS29ncTJDdEFLM0FyeUN3QUxGZ3NzQ3pvQUFRQUFBRWtBakFBTUFHWUFCd0FCQUFBQUFBQUFBQUFBQUFBQUFBUUFBM2ljbkpUYlRodFhGSVkvQjl0dGVycW9VRVJ1MEw1TXBXUk1veEFsNGNxVW9JeUtjT3B4ZXBDcVNvTTlQb2p4ek1nem1KSW42SFhmb20rUnF6NUduNkxxZGJWL0w0TWRSVUVnQlB4NzlqcjhhNjEvYldDVC85aWdWcjhML04yY0c2NngzZnpaOEIyK2FCNFozbUMvK1puaE9nOGIveGh1TUdpOE5kemtRYU5yK0JQZTFmODAvQ2xQNnI4WnZzdFcvZER3NXp5dWJ4citjc1B4citHdmVNSzdCYTdCTS80d1hHT0x3dkFkTnZuVjhBYjNzSmkxT3ZmWU1kemdhN1lOTjlrR2VreW9TSm1RTWNJeFpNS0lNMllrbEVRa3pKZ3dKR0dBSTZSTlNxV3ZHYkdRWS9UQnJ6RVJGVE5pUlJ4VDRVaUpTSWtwR1Z2RXQvTEtlYTJNUTUxbWR0ZW1Za3pNaVR4T2ljbHc1SXpJeVVnNFZaeUtpb0lYdEdoUjBoZmZnb3FTZ0pJSktRRTVNMGEwNkhESUVUM0dUQ2h4SENxU1p4YVJNNlRpbkZqNW5WbjR6dlJKeUNpTjFSa1pBL0YwNHBmSU8rUUlSNGRDdHF1Umo5WWlQTVR4bzd3OXQxeTIzeExvMTYwd1c4KzdaQk16Vno5VGRTWFZ6YmttT05hdHo5dm1CK0dLRjdoYjlXZWR5ZlU5R3VoL3BjZ25uR24rQTAwcUU1TU01N1pvRTBsQmtidVBZMS9ua0VnZCtZbVFIcS9vOElhZXptMjZkR2x6VEkrUWwvTHQwTVh4SFIyT09aQkhLTHk0TzVSaWp2a0Z4L2VFc3ZHeEUrdlBZbUlKdjFPWXVrdHhuS21PS1lWNjdwa0hxalZSaFRlZnNOK2hmRTBkcFh6NjJpTnY2VFMvVEhzV016SlZGR0k0VlMrWDJpaXRmd05UeEZTMStObGUzZnR0bU52dUxiZjRnbHc3N05XNjRPUW50MkIwM1ZTRDl6UnpycCtBbUFFNUo3TG9rek9sUmNXRmVMOG01b3dVeDRHNjkwcGJVdEcrOVBGNUxxU1NoS2tZaEdTS002UFEzOWgwRXhuMy9wcnVuYjBsQS9sN3BxZVhWZDBtaTFPdnJtYjBSdDFiM2tYVzVXUmxBaTJiYXI2aXByNjRacWI5UkR1MXlqK1NiNm5YTGVjUm9lSXVkdnREcjhBT3o5bGxqN0d5OUhVenY3enpyNFMzMkZNSFRrbGtOWlNtZlEyUENkZ2w0Q203N1BLY3ArLzFjc25HR1IrM3htYzFmNXNEOXVtd2QyMDFDOXNPKzd4Y2kvYnh6SCtKN1k3cWNUeTZQRDI3OVRRZjNFQzEzMmpmcnQ3TnJpYm5wekczYUZmYmNVek0xSE54VzZzMXVmc0Uvd01BQVAvL0FRQUEvLzl5b1ZGQUFBQUFBd0FBLy9VQUFQL09BRElBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUE9PSIpOwp9XV0+PC9zdHlsZT48c3R5bGUgdHlwZT0idGV4dC9jc3MiPjwhW0NEQVRBWy5zaGFwZSB7CiAgc2hhcGUtcmVuZGVyaW5nOiBnZW9tZXRyaWNQcmVjaXNpb247CiAgc3Ryb2tlLWxpbmVqb2luOiByb3VuZDsKfQouY29ubmVjdGlvbiB7CiAgc3Ryb2tlLWxpbmVjYXA6IHJvdW5kOwogIHN0cm9rZS1saW5lam9pbjogcm91bmQ7Cn0KLmJsZW5kIHsKICBtaXgtYmxlbmQtbW9kZTogbXVsdGlwbHk7CiAgb3BhY2l0eTogMC41Owp9CgoJCS5kMi00MTAyODc2MzM5IC5maWxsLU4xe2ZpbGw6IzBBMEYyNTt9CgkJLmQyLTQxMDI4NzYzMzkgLmZpbGwtTjJ7ZmlsbDojNjc2QzdFO30KCQkuZDItNDEwMjg3NjMzOSAuZmlsbC1OM3tmaWxsOiM5NDk5QUI7fQoJCS5kMi00MTAyODc2MzM5IC5maWxsLU40e2ZpbGw6I0NGRDJERDt9CgkJLmQyLTQxMDI4NzYzMzkgLmZpbGwtTjV7ZmlsbDojREVFMUVCO30KCQkuZDItNDEwMjg3NjMzOSAuZmlsbC1ONntmaWxsOiNFRUYxRjg7fQoJCS5kMi00MTAyODc2MzM5IC5maWxsLU43e2ZpbGw6I0ZGRkZGRjt9CgkJLmQyLTQxMDI4NzYzMzkgLmZpbGwtQjF7ZmlsbDojMEQzMkIyO30KCQkuZDItNDEwMjg3NjMzOSAuZmlsbC1CMntmaWxsOiMwRDMyQjI7fQoJCS5kMi00MTAyODc2MzM5IC5maWxsLUIze2ZpbGw6I0UzRTlGRDt9CgkJLmQyLTQxMDI4NzYzMzkgLmZpbGwtQjR7ZmlsbDojRTNFOUZEO30KCQkuZDItNDEwMjg3NjMzOSAuZmlsbC1CNXtmaWxsOiNFREYwRkQ7fQoJCS5kMi00MTAyODc2MzM5IC5maWxsLUI2e2ZpbGw6I0Y3RjhGRTt9CgkJLmQyLTQxMDI4NzYzMzkgLmZpbGwtQUEye2ZpbGw6IzRBNkZGMzt9CgkJLmQyLTQxMDI4NzYzMzkgLmZpbGwtQUE0e2ZpbGw6I0VERjBGRDt9CgkJLmQyLTQxMDI4NzYzMzkgLmZpbGwtQUE1e2ZpbGw6I0Y3RjhGRTt9CgkJLmQyLTQxMDI4NzYzMzkgLmZpbGwtQUI0e2ZpbGw6I0VERjBGRDt9CgkJLmQyLTQxMDI4NzYzMzkgLmZpbGwtQUI1e2ZpbGw6I0Y3RjhGRTt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1OMXtzdHJva2U6IzBBMEYyNTt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1OMntzdHJva2U6IzY3NkM3RTt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1OM3tzdHJva2U6Izk0OTlBQjt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1ONHtzdHJva2U6I0NGRDJERDt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1ONXtzdHJva2U6I0RFRTFFQjt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1ONntzdHJva2U6I0VFRjFGODt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1ON3tzdHJva2U6I0ZGRkZGRjt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1CMXtzdHJva2U6IzBEMzJCMjt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1CMntzdHJva2U6IzBEMzJCMjt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1CM3tzdHJva2U6I0UzRTlGRDt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1CNHtzdHJva2U6I0UzRTlGRDt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1CNXtzdHJva2U6I0VERjBGRDt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1CNntzdHJva2U6I0Y3RjhGRTt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1BQTJ7c3Ryb2tlOiM0QTZGRjM7fQoJCS5kMi00MTAyODc2MzM5IC5zdHJva2UtQUE0e3N0cm9rZTojRURGMEZEO30KCQkuZDItNDEwMjg3NjMzOSAuc3Ryb2tlLUFBNXtzdHJva2U6I0Y3RjhGRTt9CgkJLmQyLTQxMDI4NzYzMzkgLnN0cm9rZS1BQjR7c3Ryb2tlOiNFREYwRkQ7fQoJCS5kMi00MTAyODc2MzM5IC5zdHJva2UtQUI1e3N0cm9rZTojRjdGOEZFO30KCQkuZDItNDEwMjg3NjMzOSAuYmFja2dyb3VuZC1jb2xvci1OMXtiYWNrZ3JvdW5kLWNvbG9yOiMwQTBGMjU7fQoJCS5kMi00MTAyODc2MzM5IC5iYWNrZ3JvdW5kLWNvbG9yLU4ye2JhY2tncm91bmQtY29sb3I6IzY3NkM3RTt9CgkJLmQyLTQxMDI4NzYzMzkgLmJhY2tncm91bmQtY29sb3ItTjN7YmFja2dyb3VuZC1jb2xvcjojOTQ5OUFCO30KCQkuZDItNDEwMjg3NjMzOSAuYmFja2dyb3VuZC1jb2xvci1ONHtiYWNrZ3JvdW5kLWNvbG9yOiNDRkQyREQ7fQoJCS5kMi00MTAyODc2MzM5IC5iYWNrZ3JvdW5kLWNvbG9yLU41e2JhY2tncm91bmQtY29sb3I6I0RFRTFFQjt9CgkJLmQyLTQxMDI4NzYzMzkgLmJhY2tncm91bmQtY29sb3ItTjZ7YmFja2dyb3VuZC1jb2xvcjojRUVGMUY4O30KCQkuZDItNDEwMjg3NjMzOSAuYmFja2dyb3VuZC1jb2xvci1ON3tiYWNrZ3JvdW5kLWNvbG9yOiNGRkZGRkY7fQoJCS5kMi00MTAyODc2MzM5IC5iYWNrZ3JvdW5kLWNvbG9yLUIxe2JhY2tncm91bmQtY29sb3I6IzBEMzJCMjt9CgkJLmQyLTQxMDI4NzYzMzkgLmJhY2tncm91bmQtY29sb3ItQjJ7YmFja2dyb3VuZC1jb2xvcjojMEQzMkIyO30KCQkuZDItNDEwMjg3NjMzOSAuYmFja2dyb3VuZC1jb2xvci1CM3tiYWNrZ3JvdW5kLWNvbG9yOiNFM0U5RkQ7fQoJCS5kMi00MTAyODc2MzM5IC5iYWNrZ3JvdW5kLWNvbG9yLUI0e2JhY2tncm91bmQtY29sb3I6I0UzRTlGRDt9CgkJLmQyLTQxMDI4NzYzMzkgLmJhY2tncm91bmQtY29sb3ItQjV7YmFja2dyb3VuZC1jb2xvcjojRURGMEZEO30KCQkuZDItNDEwMjg3NjMzOSAuYmFja2dyb3VuZC1jb2xvci1CNntiYWNrZ3JvdW5kLWNvbG9yOiNGN0Y4RkU7fQoJCS5kMi00MTAyODc2MzM5IC5iYWNrZ3JvdW5kLWNvbG9yLUFBMntiYWNrZ3JvdW5kLWNvbG9yOiM0QTZGRjM7fQoJCS5kMi00MTAyODc2MzM5IC5iYWNrZ3JvdW5kLWNvbG9yLUFBNHtiYWNrZ3JvdW5kLWNvbG9yOiNFREYwRkQ7fQoJCS5kMi00MTAyODc2MzM5IC5iYWNrZ3JvdW5kLWNvbG9yLUFBNXtiYWNrZ3JvdW5kLWNvbG9yOiNGN0Y4RkU7fQoJCS5kMi00MTAyODc2MzM5IC5iYWNrZ3JvdW5kLWNvbG9yLUFCNHtiYWNrZ3JvdW5kLWNvbG9yOiNFREYwRkQ7fQoJCS5kMi00MTAyODc2MzM5IC5iYWNrZ3JvdW5kLWNvbG9yLUFCNXtiYWNrZ3JvdW5kLWNvbG9yOiNGN0Y4RkU7fQoJCS5kMi00MTAyODc2MzM5IC5jb2xvci1OMXtjb2xvcjojMEEwRjI1O30KCQkuZDItNDEwMjg3NjMzOSAuY29sb3ItTjJ7Y29sb3I6IzY3NkM3RTt9CgkJLmQyLTQxMDI4NzYzMzkgLmNvbG9yLU4ze2NvbG9yOiM5NDk5QUI7fQoJCS5kMi00MTAyODc2MzM5IC5jb2xvci1ONHtjb2xvcjojQ0ZEMkREO30KCQkuZDItNDEwMjg3NjMzOSAuY29sb3ItTjV7Y29sb3I6I0RFRTFFQjt9CgkJLmQyLTQxMDI4NzYzMzkgLmNvbG9yLU42e2NvbG9yOiNFRUYxRjg7fQoJCS5kMi00MTAyODc2MzM5IC5jb2xvci1ON3tjb2xvcjojRkZGRkZGO30KCQkuZDItNDEwMjg3NjMzOSAuY29sb3ItQjF7Y29sb3I6IzBEMzJCMjt9CgkJLmQyLTQxMDI4NzYzMzkgLmNvbG9yLUIye2NvbG9yOiMwRDMyQjI7fQoJCS5kMi00MTAyODc2MzM5IC5jb2xvci1CM3tjb2xvcjojRTNFOUZEO30KCQkuZDItNDEwMjg3NjMzOSAuY29sb3ItQjR7Y29sb3I6I0UzRTlGRDt9CgkJLmQyLTQxMDI4NzYzMzkgLmNvbG9yLUI1e2NvbG9yOiNFREYwRkQ7fQoJCS5kMi00MTAyODc2MzM5IC5jb2xvci1CNntjb2xvcjojRjdGOEZFO30KCQkuZDItNDEwMjg3NjMzOSAuY29sb3ItQUEye2NvbG9yOiM0QTZGRjM7fQoJCS5kMi00MTAyODc2MzM5IC5jb2xvci1BQTR7Y29sb3I6I0VERjBGRDt9CgkJLmQyLTQxMDI4NzYzMzkgLmNvbG9yLUFBNXtjb2xvcjojRjdGOEZFO30KCQkuZDItNDEwMjg3NjMzOSAuY29sb3ItQUI0e2NvbG9yOiNFREYwRkQ7fQoJCS5kMi00MTAyODc2MzM5IC5jb2xvci1BQjV7Y29sb3I6I0Y3RjhGRTt9LmFwcGVuZGl4IHRleHQudGV4dHtmaWxsOiMwQTBGMjV9Lm1key0tY29sb3ItZmctZGVmYXVsdDojMEEwRjI1Oy0tY29sb3ItZmctbXV0ZWQ6IzY3NkM3RTstLWNvbG9yLWZnLXN1YnRsZTojOTQ5OUFCOy0tY29sb3ItY2FudmFzLWRlZmF1bHQ6I0ZGRkZGRjstLWNvbG9yLWNhbnZhcy1zdWJ0bGU6I0VFRjFGODstLWNvbG9yLWJvcmRlci1kZWZhdWx0OiMwRDMyQjI7LS1jb2xvci1ib3JkZXItbXV0ZWQ6IzBEMzJCMjstLWNvbG9yLW5ldXRyYWwtbXV0ZWQ6I0VFRjFGODstLWNvbG9yLWFjY2VudC1mZzojMEQzMkIyOy0tY29sb3ItYWNjZW50LWVtcGhhc2lzOiMwRDMyQjI7LS1jb2xvci1hdHRlbnRpb24tc3VidGxlOiM2NzZDN0U7LS1jb2xvci1kYW5nZXItZmc6cmVkO30uc2tldGNoLW92ZXJsYXktQjF7ZmlsbDp1cmwoI3N0cmVha3MtZGFya2VyLWQyLTQxMDI4NzYzMzkpO21peC1ibGVuZC1tb2RlOmxpZ2h0ZW59LnNrZXRjaC1vdmVybGF5LUIye2ZpbGw6dXJsKCNzdHJlYWtzLWRhcmtlci1kMi00MTAyODc2MzM5KTttaXgtYmxlbmQtbW9kZTpsaWdodGVufS5za2V0Y2gtb3ZlcmxheS1CM3tmaWxsOnVybCgjc3RyZWFrcy1icmlnaHQtZDItNDEwMjg3NjMzOSk7bWl4LWJsZW5kLW1vZGU6ZGFya2VufS5za2V0Y2gtb3ZlcmxheS1CNHtmaWxsOnVybCgjc3RyZWFrcy1icmlnaHQtZDItNDEwMjg3NjMzOSk7bWl4LWJsZW5kLW1vZGU6ZGFya2VufS5za2V0Y2gtb3ZlcmxheS1CNXtmaWxsOnVybCgjc3RyZWFrcy1icmlnaHQtZDItNDEwMjg3NjMzOSk7bWl4LWJsZW5kLW1vZGU6ZGFya2VufS5za2V0Y2gtb3ZlcmxheS1CNntmaWxsOnVybCgjc3RyZWFrcy1icmlnaHQtZDItNDEwMjg3NjMzOSk7bWl4LWJsZW5kLW1vZGU6ZGFya2VufS5za2V0Y2gtb3ZlcmxheS1BQTJ7ZmlsbDp1cmwoI3N0cmVha3MtZGFyay1kMi00MTAyODc2MzM5KTttaXgtYmxlbmQtbW9kZTpvdmVybGF5fS5za2V0Y2gtb3ZlcmxheS1BQTR7ZmlsbDp1cmwoI3N0cmVha3MtYnJpZ2h0LWQyLTQxMDI4NzYzMzkpO21peC1ibGVuZC1tb2RlOmRhcmtlbn0uc2tldGNoLW92ZXJsYXktQUE1e2ZpbGw6dXJsKCNzdHJlYWtzLWJyaWdodC1kMi00MTAyODc2MzM5KTttaXgtYmxlbmQtbW9kZTpkYXJrZW59LnNrZXRjaC1vdmVybGF5LUFCNHtmaWxsOnVybCgjc3RyZWFrcy1icmlnaHQtZDItNDEwMjg3NjMzOSk7bWl4LWJsZW5kLW1vZGU6ZGFya2VufS5za2V0Y2gtb3ZlcmxheS1BQjV7ZmlsbDp1cmwoI3N0cmVha3MtYnJpZ2h0LWQyLTQxMDI4NzYzMzkpO21peC1ibGVuZC1tb2RlOmRhcmtlbn0uc2tldGNoLW92ZXJsYXktTjF7ZmlsbDp1cmwoI3N0cmVha3MtZGFya2VyLWQyLTQxMDI4NzYzMzkpO21peC1ibGVuZC1tb2RlOmxpZ2h0ZW59LnNrZXRjaC1vdmVybGF5LU4ye2ZpbGw6dXJsKCNzdHJlYWtzLWRhcmstZDItNDEwMjg3NjMzOSk7bWl4LWJsZW5kLW1vZGU6b3ZlcmxheX0uc2tldGNoLW92ZXJsYXktTjN7ZmlsbDp1cmwoI3N0cmVha3Mtbm9ybWFsLWQyLTQxMDI4NzYzMzkpO21peC1ibGVuZC1tb2RlOmNvbG9yLWJ1cm59LnNrZXRjaC1vdmVybGF5LU40e2ZpbGw6dXJsKCNzdHJlYWtzLW5vcm1hbC1kMi00MTAyODc2MzM5KTttaXgtYmxlbmQtbW9kZTpjb2xvci1idXJufS5za2V0Y2gtb3ZlcmxheS1ONXtmaWxsOnVybCgjc3RyZWFrcy1icmlnaHQtZDItNDEwMjg3NjMzOSk7bWl4LWJsZW5kLW1vZGU6ZGFya2VufS5za2V0Y2gtb3ZlcmxheS1ONntmaWxsOnVybCgjc3RyZWFrcy1icmlnaHQtZDItNDEwMjg3NjMzOSk7bWl4LWJsZW5kLW1vZGU6ZGFya2VufS5za2V0Y2gtb3ZlcmxheS1ON3tmaWxsOnVybCgjc3RyZWFrcy1icmlnaHQtZDItNDEwMjg3NjMzOSk7bWl4LWJsZW5kLW1vZGU6ZGFya2VufS5saWdodC1jb2Rle2Rpc3BsYXk6IGJsb2NrfS5kYXJrLWNvZGV7ZGlzcGxheTogbm9uZX1dXT48L3N0eWxlPjxnIGNsYXNzPSJaM2x0WDJ4aGRXNWphQT09Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIxMzA4LjAwMDAwMCIgeT0iMTIuMDAwMDAwIiB3aWR0aD0iMTczMS4wMDAwMDAiIGhlaWdodD0iMTY2LjAwMDAwMCIgc3Ryb2tlPSIjOTI0MDBlIiBmaWxsPSIjZmVmM2M3IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIyMTczLjUwMDAwMCIgeT0iNDUuMDAwMDAwIiBmaWxsPSIjNzgzNTBmIiBjbGFzcz0idGV4dC1ib2xkIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToyOHB4Ij5HeW0gTGF1bmNoIFNlY3JldHMgKDIwMjApPC90ZXh0PjwvZz48ZyBjbGFzcz0iYjJabVpYSnpYMlp2ZFc1a1lYUnBiMjQ9Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSI4NjcuMDAwMDAwIiB5PSI0NTkuMDAwMDAwIiB3aWR0aD0iNTk5LjAwMDAwMCIgaGVpZ2h0PSI2MjAuMDAwMDAwIiBzdHJva2U9IiMxZTQwYWYiIGZpbGw9IiNkYmVhZmUiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjExNjYuNTAwMDAwIiB5PSI0OTIuMDAwMDAwIiBmaWxsPSIjMWUzYThhIiBjbGFzcz0idGV4dC1ib2xkIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToyOHB4Ij4xMDBNIE9mZmVycyAtIEZvdW5kYXRpb248L3RleHQ+PC9nPjxnIGNsYXNzPSJaM052WDJOdmJYQnZibVZ1ZEhNPSI+PGcgY2xhc3M9InNoYXBlIiA+PHJlY3QgeD0iMTIuMDAwMDAwIiB5PSIxMzIwLjAwMDAwMCIgd2lkdGg9IjIxMDYuMDAwMDAwIiBoZWlnaHQ9IjE2Ni4wMDAwMDAiIHN0cm9rZT0iIzFlNDBhZiIgZmlsbD0iI2RiZWFmZSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgLz48L2c+PHRleHQgeD0iMTA2NS4wMDAwMDAiIHk9IjEzNTMuMDAwMDAwIiBmaWxsPSIjMWUzYThhIiBjbGFzcz0idGV4dCIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MjhweCI+R3JhbmQgU2xhbSBPZmZlciAtIENvbXBvbmVudHM8L3RleHQ+PC9nPjxnIGNsYXNzPSJaM052Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIxMjQzLjAwMDAwMCIgeT0iMTg3Mi4wMDAwMDAiIHdpZHRoPSIzMjAuMDAwMDAwIiBoZWlnaHQ9IjczLjAwMDAwMCIgc3Ryb2tlPSIjMWUzYThhIiBmaWxsPSIjMWU0MGFmIiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIxNDAzLjAwMDAwMCIgeT0iMTkxNi41MDAwMDAiIGZpbGw9IiNmZmZmZmYiIGNsYXNzPSJ0ZXh0LWJvbGQiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjIycHgiPkdSQU5EIFNMQU0gT0ZGRVI8L3RleHQ+PC9nPjxnIGNsYXNzPSJiR1ZoWkhNPSI+PGcgY2xhc3M9InNoYXBlIiA+PHJlY3QgeD0iMTE0OC4wMDAwMDAiIHk9IjIxMjEuMDAwMDAwIiB3aWR0aD0iMTAzNS4wMDAwMDAiIGhlaWdodD0iNjkwLjAwMDAwMCIgc3Ryb2tlPSIjMTY2NTM0IiBmaWxsPSIjZGNmY2U3IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIxNjY1LjUwMDAwMCIgeT0iMjE1NC4wMDAwMDAiIGZpbGw9IiMxNDUzMmQiIGNsYXNzPSJ0ZXh0LWJvbGQiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjI4cHgiPjEwME0gTGVhZHMgLSBEaXN0cmlidXRpb248L3RleHQ+PC9nPjxnIGNsYXNzPSJiR1ZoWkE9PSI+PGcgY2xhc3M9InNoYXBlIiA+PHJlY3QgeD0iMTU4NS4wMDAwMDAiIHk9IjMwOTcuMDAwMDAwIiB3aWR0aD0iMjIzLjAwMDAwMCIgaGVpZ2h0PSI2Ni4wMDAwMDAiIHN0cm9rZT0iIzE0NTMyZCIgZmlsbD0iIzE2NjUzNCIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgLz48L2c+PHRleHQgeD0iMTY5Ni41MDAwMDAiIHk9IjMxMzUuNTAwMDAwIiBmaWxsPSIjZmZmZmZmIiBjbGFzcz0idGV4dC1ib2xkIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5MRUFEIChhY3F1aXJlZCBwcm9zcGVjdCk8L3RleHQ+PC9nPjxnIGNsYXNzPSJiVzl1WlhsZmJXOWtaV3hmWTI5dGNHOXVaVzUwY3c9PSI+PGcgY2xhc3M9InNoYXBlIiA+PHJlY3QgeD0iMTU5NS4wMDAwMDAiIHk9IjMzMzkuMDAwMDAwIiB3aWR0aD0iNTcxLjAwMDAwMCIgaGVpZ2h0PSI2NDAuMDAwMDAwIiBzdHJva2U9IiM5ZDE3NGQiIGZpbGw9IiNmY2U3ZjMiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjE4ODAuNTAwMDAwIiB5PSIzMzcyLjAwMDAwMCIgZmlsbD0iIzgzMTg0MyIgY2xhc3M9InRleHQtYm9sZCIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MjhweCI+MTAwTSBNb25leSBNb2RlbHMgLSBTZXF1ZW5jZTwvdGV4dD48L2c+PGcgY2xhc3M9IlkyRnphRjlqZVdOc1pRPT0iPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjIzMzAuMDAwMDAwIiB5PSI0MjA1LjAwMDAwMCIgd2lkdGg9IjQwNC4wMDAwMDAiIGhlaWdodD0iNzEuMDAwMDAwIiBzdHJva2U9IiM4MzE4NDMiIGZpbGw9IiM5ZDE3NGQiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjI1MzIuMDAwMDAwIiB5PSI0MjQ3LjUwMDAwMCIgZmlsbD0iI2ZmZmZmZiIgY2xhc3M9InRleHQtYm9sZCIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MjBweCI+MzAtREFZIENBU0ggQ1lDTEUgLSAoQ0FDICZsdDsgMzAtZGF5IGNhc2gpPC90ZXh0PjwvZz48ZyBjbGFzcz0iYzJOaGJHVT0iPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjI4MjAuMDAwMDAwIiB5PSI0NDk3LjAwMDAwMCIgd2lkdGg9IjMxMy4wMDAwMDAiIGhlaWdodD0iNzMuMDAwMDAwIiBzdHJva2U9IiMwMDAwMDAiIGZpbGw9IiMwMDAwMDAiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjI5NzYuNTAwMDAwIiB5PSI0NTQxLjUwMDAwMCIgZmlsbD0iI2ZmZmZmZiIgY2xhc3M9InRleHQtYm9sZCIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MjJweCI+U0NBTEUgLSAocG91ciBjYXNoIGludG8gYWRzKTwvdGV4dD48L2c+PGcgY2xhc3M9ImJHOXpkRjlqYUdGd2RHVnljdz09Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIyODcxLjAwMDAwMCIgeT0iMTMyMC4wMDAwMDAiIHdpZHRoPSIxMTU1LjAwMDAwMCIgaGVpZ2h0PSIxNjYuMDAwMDAwIiBzdHJva2U9IiM2YjIxYTgiIGZpbGw9IiNmM2U4ZmYiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjM0NDguNTAwMDAwIiB5PSIxMzUzLjAwMDAwMCIgZmlsbD0iIzU4MWM4NyIgY2xhc3M9InRleHQtYm9sZCIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MjhweCI+MTAwTSBMb3N0IENoYXB0ZXJzIC0gRGlhZ25vc3RpY3M8L3RleHQ+PC9nPjxnIGNsYXNzPSJiR1ZuWlc1ayI+PGcgY2xhc3M9InNoYXBlIiA+PHJlY3QgeD0iMzA1OS4wMDAwMDAiIHk9IjE0LjAwMDAwMCIgd2lkdGg9IjE1MDEuMDAwMDAwIiBoZWlnaHQ9IjE2Ni4wMDAwMDAiIHN0cm9rZT0iIzljYTNhZiIgZmlsbD0iI2ZmZmZmZiIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgLz48L2c+PHRleHQgeD0iMzgwOS41MDAwMDAiIHk9IjQ3LjAwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MjhweCI+bGVnZW5kPC90ZXh0PjwvZz48ZyBjbGFzcz0iWjNsdFgyeGhkVzVqYUM0NFgzTjBaWEJmYjJabVpYST0iPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjEzNTguMDAwMDAwIiB5PSI2Mi4wMDAwMDAiIHdpZHRoPSIyMTIuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIxNDY0LjAwMDAwMCIgeT0iMTAwLjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij44LVN0ZXAgSXJyZXNpc3RpYmxlIE9mZmVyPC90ZXh0PjwvZz48ZyBjbGFzcz0iWjNsdFgyeGhkVzVqYUM1c1pXRmtYM05qY21GdFlteGxjZz09Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIxNTkwLjAwMDAwMCIgeT0iNjIuMDAwMDAwIiB3aWR0aD0iMjM4LjAwMDAwMCIgaGVpZ2h0PSI2Ni4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0iI0VERjBGRCIgY2xhc3M9IiBzdHJva2UtQjEgZmlsbC1CNSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgLz48L2c+PHRleHQgeD0iMTcwOS4wMDAwMDAiIHk9IjEwMC41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+TGVhZCBHZW5lcmF0aW9uIFNjcmFtYmxlcjwvdGV4dD48L2c+PGcgY2xhc3M9IlozbHRYMnhoZFc1amFDNTNaV1JrYVc1blgyTmhhMlU9Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIxODQ4LjAwMDAwMCIgeT0iNjIuMDAwMDAwIiB3aWR0aD0iMTkyLjAwMDAwMCIgaGVpZ2h0PSI2Ni4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0iI0VERjBGRCIgY2xhc3M9IiBzdHJva2UtQjEgZmlsbC1CNSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgLz48L2c+PHRleHQgeD0iMTk0NC4wMDAwMDAiIHk9IjEwMC41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+V2VkZGluZyBDYWtlIE1vZGVsPC90ZXh0PjwvZz48ZyBjbGFzcz0iWjNsdFgyeGhkVzVqYUM1amJHOXpaWEk9Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIyMDYwLjAwMDAwMCIgeT0iNjIuMDAwMDAwIiB3aWR0aD0iMjE0LjAwMDAwMCIgaGVpZ2h0PSI2Ni4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0iI0VERjBGRCIgY2xhc3M9IiBzdHJva2UtQjEgZmlsbC1CNSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgLz48L2c+PHRleHQgeD0iMjE2Ny4wMDAwMDAiIHk9IjEwMC41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+Qy1MLU8tUy1FLVIgU2FsZXMgU2NyaXB0PC90ZXh0PjwvZz48ZyBjbGFzcz0iWjNsdFgyeGhkVzVqYUM1alptRT0iPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjIyOTQuMDAwMDAwIiB5PSI2Mi4wMDAwMDAiIHdpZHRoPSIyNDAuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIyNDE0LjAwMDAwMCIgeT0iMTAwLjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5DbGllbnQtRmluYW5jZWQgQWNxdWlzaXRpb248L3RleHQ+PC9nPjxnIGNsYXNzPSJaM2x0WDJ4aGRXNWphQzVtYVhabFgyaHZjbk5sYldWdSI+PGcgY2xhc3M9InNoYXBlIiA+PHJlY3QgeD0iMjU1NC4wMDAwMDAiIHk9IjYyLjAwMDAwMCIgd2lkdGg9IjIyMS4wMDAwMDAiIGhlaWdodD0iNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9IiNFREYwRkQiIGNsYXNzPSIgc3Ryb2tlLUIxIGZpbGwtQjUiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjI2NjQuNTAwMDAwIiB5PSIxMDAuNTAwMDAwIiBmaWxsPSIjMEEwRjI1IiBjbGFzcz0idGV4dC1ib2xkIGZpbGwtTjEiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPjUgSG9yc2VtZW4gb2YgUmV0ZW50aW9uPC90ZXh0PjwvZz48ZyBjbGFzcz0iWjNsdFgyeGhkVzVqYUM1d2NtOW1hWFJmYkdWMlpYSnoiPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjI3OTUuMDAwMDAwIiB5PSI2Mi4wMDAwMDAiIHdpZHRoPSIxOTQuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIyODkyLjAwMDAwMCIgeT0iMTAwLjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij4zIE1hY3JvIFByb2ZpdCBMZXZlcnM8L3RleHQ+PC9nPjxnIGNsYXNzPSJiMlptWlhKelgyWnZkVzVrWVhScGIyNHViV0Z5YTJWMCI+PGcgY2xhc3M9InNoYXBlIiA+PHJlY3QgeD0iOTc2LjAwMDAwMCIgeT0iNTA5LjAwMDAwMCIgd2lkdGg9IjIxNS4wMDAwMDAiIGhlaWdodD0iNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9IiNFREYwRkQiIGNsYXNzPSIgc3Ryb2tlLUIxIGZpbGwtQjUiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjEwODMuNTAwMDAwIiB5PSI1NDcuNTAwMDAwIiBmaWxsPSIjMEEwRjI1IiBjbGFzcz0idGV4dC1ib2xkIGZpbGwtTjEiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPlRpbnkgTWFya2V0IC8gQmlnIE1vbmV5PC90ZXh0PjwvZz48ZyBjbGFzcz0iYjJabVpYSnpYMlp2ZFc1a1lYUnBiMjR1WVhaaGRHRnkiPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9Ijk5My4wMDAwMDAiIHk9IjczNi4wMDAwMDAiIHdpZHRoPSIxODEuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIxMDgzLjUwMDAwMCIgeT0iNzc0LjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5BdmF0YXIgKDV4IE1ldGhvZCk8L3RleHQ+PC9nPjxnIGNsYXNzPSJiMlptWlhKelgyWnZkVzVrWVhScGIyNHVkbUZzZFdWZlpYRT0iPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjkxNy4wMDAwMDAiIHk9Ijk2My4wMDAwMDAiIHdpZHRoPSI0OTkuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIxMTY2LjUwMDAwMCIgeT0iMTAwMS41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+VmFsdWUgRXF1YXRpb24gLSAoRHJlYW0gT3V0Y29tZSDDlyBMaWtlbGlob29kKSAtIC8oVGltZSDDlyBFZmZvcnQpPC90ZXh0PjwvZz48ZyBjbGFzcz0iWjNOdlgyTnZiWEJ2Ym1WdWRITXVaR1ZzYVhabGNubGZZM1ZpWlE9PSI+PGcgY2xhc3M9InNoYXBlIiA+PHJlY3QgeD0iNjIuMDAwMDAwIiB5PSIxMzcwLjAwMDAwMCIgd2lkdGg9IjI1OC4wMDAwMDAiIGhlaWdodD0iNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9IiNFREYwRkQiIGNsYXNzPSIgc3Ryb2tlLUIxIGZpbGwtQjUiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjE5MS4wMDAwMDAiIHk9IjE0MDguNTAwMDAwIiBmaWxsPSIjMEEwRjI1IiBjbGFzcz0idGV4dC1ib2xkIGZpbGwtTjEiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPkRlbGl2ZXJ5IEN1YmUgLSAoNiBkaW1lbnNpb25zKTwvdGV4dD48L2c+PGcgY2xhc3M9IlozTnZYMk52YlhCdmJtVnVkSE11ZEhKcGJWOXpkR0ZqYXc9PSI+PGcgY2xhc3M9InNoYXBlIiA+PHJlY3QgeD0iMzQwLjAwMDAwMCIgeT0iMTM3MC4wMDAwMDAiIHdpZHRoPSIyOTIuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSI0ODYuMDAwMDAwIiB5PSIxNDA4LjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5UcmltICZhbXA7IFN0YWNrIC0gKHZhbHVlIMOXIGNvc3QgbWF0cml4KTwvdGV4dD48L2c+PGcgY2xhc3M9IlozTnZYMk52YlhCdmJtVnVkSE11WW05dWRYTmxjdz09Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSI2NTIuMDAwMDAwIiB5PSIxMzcwLjAwMDAwMCIgd2lkdGg9IjE4OS4wMDAwMDAiIGhlaWdodD0iNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9IiNFREYwRkQiIGNsYXNzPSIgc3Ryb2tlLUIxIGZpbGwtQjUiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9Ijc0Ni41MDAwMDAiIHk9IjE0MDguNTAwMDAwIiBmaWxsPSIjMEEwRjI1IiBjbGFzcz0idGV4dC1ib2xkIGZpbGwtTjEiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPlVuYmVhdGFibGUgQm9udXNlczwvdGV4dD48L2c+PGcgY2xhc3M9IlozTnZYMk52YlhCdmJtVnVkSE11YzJOaGNtTnBkSGs9Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSI4NjEuMDAwMDAwIiB5PSIxMzcwLjAwMDAwMCIgd2lkdGg9IjE0Ni4wMDAwMDAiIGhlaWdodD0iNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9IiNFREYwRkQiIGNsYXNzPSIgc3Ryb2tlLUIxIGZpbGwtQjUiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjkzNC4wMDAwMDAiIHk9IjE0MDguNTAwMDAwIiBmaWxsPSIjMEEwRjI1IiBjbGFzcz0idGV4dC1ib2xkIGZpbGwtTjEiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPlNjYXJjaXR5IFN0YWNrPC90ZXh0PjwvZz48ZyBjbGFzcz0iWjNOdlgyTnZiWEJ2Ym1WdWRITXVkWEpuWlc1amVRPT0iPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjEwMjcuMDAwMDAwIiB5PSIxMzcwLjAwMDAwMCIgd2lkdGg9IjE3My4wMDAwMDAiIGhlaWdodD0iNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9IiNFREYwRkQiIGNsYXNzPSIgc3Ryb2tlLUIxIGZpbGwtQjUiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjExMTMuNTAwMDAwIiB5PSIxNDA4LjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5FdmVyeWRheSBVcmdlbmN5PC90ZXh0PjwvZz48ZyBjbGFzcz0iWjNOdlgyTnZiWEJ2Ym1WdWRITXVaM1ZoY21GdWRHVmxjdz09Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIxMjIwLjAwMDAwMCIgeT0iMTM3MC4wMDAwMDAiIHdpZHRoPSIzNzguMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIxNDA5LjAwMDAwMCIgeT0iMTQwOC41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+R29kLU1vZGUgR3VhcmFudGVlcyAtICg0IHR5cGVzICsgMTMgZXhhbXBsZXMpPC90ZXh0PjwvZz48ZyBjbGFzcz0iWjNOdlgyTnZiWEJ2Ym1WdWRITXViV0ZuYVdNPSI+PGcgY2xhc3M9InNoYXBlIiA+PHJlY3QgeD0iMTYxOC4wMDAwMDAiIHk9IjEzNzAuMDAwMDAwIiB3aWR0aD0iNDUwLjAwMDAwMCIgaGVpZ2h0PSI2Ni4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0iI0VERjBGRCIgY2xhc3M9IiBzdHJva2UtQjEgZmlsbC1CNSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgLz48L2c+PHRleHQgeD0iMTg0My4wMDAwMDAiIHk9IjE0MDguNTAwMDAwIiBmaWxsPSIjMEEwRjI1IiBjbGFzcz0idGV4dC1ib2xkIGZpbGwtTjEiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPk1BR0lDIE5hbWluZyAtIChNYWduZXQvQXZhdGFyL0dvYWwvSW50ZXJ2YWwvQ29udGFpbmVyKTwvdGV4dD48L2c+PGcgY2xhc3M9ImJHVmhaSE11WTI5eVpWOW1iM1Z5Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIxMzE1LjAwMDAwMCIgeT0iMjE3MS4wMDAwMDAiIHdpZHRoPSIzODMuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIxNTA2LjUwMDAwMCIgeT0iMjIwOS41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+Q29yZSBGb3VyIENoYW5uZWxzIC0gKFdhcm0vQ29udGVudC9Db2xkL1BhaWQpPC90ZXh0PjwvZz48ZyBjbGFzcz0iYkdWaFpITXViR1ZoWkY5dFlXZHVaWFE9Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIxMjM5LjAwMDAwMCIgeT0iMjQ1OC4wMDAwMDAiIHdpZHRoPSIyMjAuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIxMzQ5LjAwMDAwMCIgeT0iMjQ5Ni41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+TGVhZCBNYWduZXQgRnJhbWV3b3JrPC90ZXh0PjwvZz48ZyBjbGFzcz0iYkdWaFpITXVjblZzWlY4eE1EQT0iPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjEzMzkuMDAwMDAwIiB5PSIyNjk1LjAwMDAwMCIgd2lkdGg9IjEyMi4wMDAwMDAiIGhlaWdodD0iNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9IiNFREYwRkQiIGNsYXNzPSIgc3Ryb2tlLUIxIGZpbGwtQjUiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjE0MDAuMDAwMDAwIiB5PSIyNzMzLjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5SdWxlIG9mIDEwMDwvdGV4dD48L2c+PGcgY2xhc3M9ImJHVmhaSE11YkdWaFpGOW5aWFIwWlhKeiI+PGcgY2xhc3M9InNoYXBlIiA+PHJlY3QgeD0iMTQ3OS4wMDAwMDAiIHk9IjI0NTguMDAwMDAwIiB3aWR0aD0iMzY3LjAwMDAwMCIgaGVpZ2h0PSI2Ni4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0iI0VERjBGRCIgY2xhc3M9IiBzdHJva2UtQjEgZmlsbC1CNSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgLz48L2c+PHRleHQgeD0iMTY2Mi41MDAwMDAiIHk9IjI0OTYuNTAwMDAwIiBmaWxsPSIjMEEwRjI1IiBjbGFzcz0idGV4dC1ib2xkIGZpbGwtTjEiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPjQgTGVhZCBHZXR0ZXJzIC0gKEN1c3QvQWZmaWwvRW1wbG95ZWUvQWdlbmN5KTwvdGV4dD48L2c+PGcgY2xhc3M9ImJHVmhaSE11Y21WbVpYSnlZV3h6Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIxODY2LjAwMDAwMCIgeT0iMjQ1OC4wMDAwMDAiIHdpZHRoPSIxNzkuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIxOTU1LjUwMDAwMCIgeT0iMjQ5Ni41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+NyBSZWZlcnJhbCBNZXRob2RzPC90ZXh0PjwvZz48ZyBjbGFzcz0iYkdWaFpITXVjWFZoYkdsbWFXVnkiPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjE5MjQuMDAwMDAwIiB5PSIyNjk1LjAwMDAwMCIgd2lkdGg9IjIwOS4wMDAwMDAiIGhlaWdodD0iNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9IiNFREYwRkQiIGNsYXNzPSIgc3Ryb2tlLUIxIGZpbGwtQjUiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjIwMjguNTAwMDAwIiB5PSIyNzMzLjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5PbmUtUXVlc3Rpb24gUXVhbGlmaWVyPC90ZXh0PjwvZz48ZyBjbGFzcz0iYlc5dVpYbGZiVzlrWld4ZlkyOXRjRzl1Wlc1MGN5NWhkSFJ5WVdOMGFXOXUiPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjE3MjguMDAwMDAwIiB5PSIzMzg5LjAwMDAwMCIgd2lkdGg9IjI5NC4wMDAwMDAiIGhlaWdodD0iNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9IiNFREYwRkQiIGNsYXNzPSIgc3Ryb2tlLUIxIGZpbGwtQjUiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjE4NzUuMDAwMDAwIiB5PSIzNDI3LjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5TdGFnZSBJOiBBdHRyYWN0aW9uIE9mZmVycyAtICg2IHR5cGVzKTwvdGV4dD48L2c+PGcgY2xhc3M9ImJXOXVaWGxmYlc5a1pXeGZZMjl0Y0c5dVpXNTBjeTUxY0hObGJHdz0iPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjE2NDUuMDAwMDAwIiB5PSIzNjI2LjAwMDAwMCIgd2lkdGg9IjIxNS4wMDAwMDAiIGhlaWdodD0iNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9IiNFREYwRkQiIGNsYXNzPSIgc3Ryb2tlLUIxIGZpbGwtQjUiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjE3NTIuNTAwMDAwIiB5PSIzNjY0LjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5TdGFnZSBJSTogVXBzZWxsICg0IHR5cGVzKTwvdGV4dD48L2c+PGcgY2xhc3M9ImJXOXVaWGxmYlc5a1pXeGZZMjl0Y0c5dVpXNTBjeTVrYjNkdWMyVnNiQT09Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIxODgwLjAwMDAwMCIgeT0iMzYyNi4wMDAwMDAiIHdpZHRoPSIyMzYuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIxOTk4LjAwMDAwMCIgeT0iMzY2NC41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+U3RhZ2UgSUk6IERvd25zZWxsICgzIHR5cGVzKTwvdGV4dD48L2c+PGcgY2xhc3M9ImJXOXVaWGxmYlc5a1pXeGZZMjl0Y0c5dVpXNTBjeTVqYjI1MGFXNTFhWFI1Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIxNzQ5LjAwMDAwMCIgeT0iMzg2My4wMDAwMDAiIHdpZHRoPSIyNTEuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIxODc0LjUwMDAwMCIgeT0iMzkwMS41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+U3RhZ2UgSUlJOiBDb250aW51aXR5ICgzIHR5cGVzKTwvdGV4dD48L2c+PGcgY2xhc3M9ImJHOXpkRjlqYUdGd2RHVnljeTV3WlhKbWIzSnRZVzVqWlY5a2FXRnRiMjVrIj48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIyOTIxLjAwMDAwMCIgeT0iMTM3MC4wMDAwMDAiIHdpZHRoPSI0MjEuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIzMTMxLjUwMDAwMCIgeT0iMTQwOC41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+UGVyZm9ybWFuY2UgRGlhbW9uZCAtIChIb29rL0F2YXRhci9Qcm9taXNlL1ByaWNlKTwvdGV4dD48L2c+PGcgY2xhc3M9ImJHOXpkRjlqYUdGd2RHVnljeTVzZEdkd1gyMWhkR2c9Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIzMzYyLjAwMDAwMCIgeT0iMTM3MC4wMDAwMDAiIHdpZHRoPSIyMDEuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIzNDYyLjUwMDAwMCIgeT0iMTQwOC41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+TFRHUCAvIENBQyAvIFBQRCBNYXRoPC90ZXh0PjwvZz48ZyBjbGFzcz0iYkc5emRGOWphR0Z3ZEdWeWN5NWtjMTl6ZVhOMFpXMD0iPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjM1ODMuMDAwMDAwIiB5PSIxMzcwLjAwMDAwMCIgd2lkdGg9IjM5My4wMDAwMDAiIGhlaWdodD0iNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9IiNFREYwRkQiIGNsYXNzPSIgc3Ryb2tlLUIxIGZpbGwtQjUiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIC8+PC9nPjx0ZXh0IHg9IjM3NzkuNTAwMDAwIiB5PSIxNDA4LjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij4zRHMgU3lzdGVtIC0gKERvY3VtZW50L0RlbW9uc3RyYXRlL0R1cGxpY2F0ZSk8L3RleHQ+PC9nPjxnIGNsYXNzPSJiR1ZuWlc1a0xuUnBkR3hsIj48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIzMTA5LjAwMDAwMCIgeT0iNjQuMDAwMDAwIiB3aWR0aD0iOTYuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIzMTU3LjAwMDAwMCIgeT0iMTAyLjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5MZWdlbmQ8L3RleHQ+PC9nPjxnIGNsYXNzPSJiR1ZuWlc1a0xuTnZiR2xrIj48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIzMjI1LjAwMDAwMCIgeT0iNjQuMDAwMDAwIiB3aWR0aD0iMzI3LjAwMDAwMCIgaGVpZ2h0PSI2Ni4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0iI0VERjBGRCIgY2xhc3M9IiBzdHJva2UtQjEgZmlsbC1CNSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgLz48L2c+PHRleHQgeD0iMzM4OC41MDAwMDAiIHk9IjEwMi41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+U29saWQgYXJyb3cgPSBkZXBlbmRlbmN5IC8gcHJlcmVxdWlzaXRlPC90ZXh0PjwvZz48ZyBjbGFzcz0iYkdWblpXNWtMbVJoYzJobFpBPT0iPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjM1NzIuMDAwMDAwIiB5PSI2NC4wMDAwMDAiIHdpZHRoPSIzNDEuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSIzNzQyLjUwMDAwMCIgeT0iMTAyLjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5EYXNoZWQgYXJyb3cgPSBkaWFnbm9zdGljIC8gbWVhc3VyZW1lbnQ8L3RleHQ+PC9nPjxnIGNsYXNzPSJiR1ZuWlc1a0xtSnZlREU9Ij48ZyBjbGFzcz0ic2hhcGUiID48cmVjdCB4PSIzOTMzLjAwMDAwMCIgeT0iNjQuMDAwMDAwIiB3aWR0aD0iMjY3LjAwMDAwMCIgaGVpZ2h0PSI2Ni4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0iI0VERjBGRCIgY2xhc3M9IiBzdHJva2UtQjEgZmlsbC1CNSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgLz48L2c+PHRleHQgeD0iNDA2Ni41MDAwMDAiIHk9IjEwMi41MDAwMDAiIGZpbGw9IiMwQTBGMjUiIGNsYXNzPSJ0ZXh0LWJvbGQgZmlsbC1OMSIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+U29saWQgY29sb3IgYm94ID0gYm9vayBjYXRlZ29yeTwvdGV4dD48L2c+PGcgY2xhc3M9ImJHVm5aVzVrTG1KdmVEST0iPjxnIGNsYXNzPSJzaGFwZSIgPjxyZWN0IHg9IjQyMjAuMDAwMDAwIiB5PSI2NC4wMDAwMDAiIHdpZHRoPSIyOTAuMDAwMDAwIiBoZWlnaHQ9IjY2LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSIjRURGMEZEIiBjbGFzcz0iIHN0cm9rZS1CMSBmaWxsLUI1IiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiAvPjwvZz48dGV4dCB4PSI0MzY1LjAwMDAwMCIgeT0iMTAyLjUwMDAwMCIgZmlsbD0iIzBBMEYyNSIgY2xhc3M9InRleHQtYm9sZCBmaWxsLU4xIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5Cb2xkIGRhcmsgYm94ID0ga2V5IG91dHB1dCBhcnRpZmFjdDwvdGV4dD48L2c+PGcgY2xhc3M9IktHZDViVjlzWVhWdVkyZ3VPRjl6ZEdWd1gyOW1abVZ5SUMwbVozUTdJRzltWm1WeWMxOW1iM1Z1WkdGMGFXOXVMblpoYkhWbFgyVnhLVnN3WFE9PSI+PG1hcmtlciBpZD0ibWstZDItNDEwMjg3NjMzOS0zNDg4Mzc4MTM0IiBtYXJrZXJXaWR0aD0iMTAuMDAwMDAwIiBtYXJrZXJIZWlnaHQ9IjEyLjAwMDAwMCIgcmVmWD0iNy4wMDAwMDAiIHJlZlk9IjYuMDAwMDAwIiB2aWV3Qm94PSIwLjAwMDAwMCAwLjAwMDAwMCAxMC4wMDAwMDAgMTIuMDAwMDAwIiBvcmllbnQ9ImF1dG8iIG1hcmtlclVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+IDxwb2x5Z29uIHBvaW50cz0iMC4wMDAwMDAsMC4wMDAwMDAgMTAuMDAwMDAwLDYuMDAwMDAwIDAuMDAwMDAwLDEyLjAwMDAwMCIgZmlsbD0iIzBEMzJCMiIgY2xhc3M9ImNvbm5lY3Rpb24gZmlsbC1CMSIgc3Ryb2tlLXdpZHRoPSIyIiAvPiA8L21hcmtlcj48cGF0aCBkPSJNIDE0NjQuOTE2MDE2IDEzMC4wMDAwMDAgTCAxNDY0LjkxNjAxNiAyMTMuMDAwMDAwIFMgMTQ2NC45MTYwMTYgMjIzLjAwMDAwMCAxNDU0LjkxNjAxNiAyMjMuMDAwMDAwIEwgMTI2MC4xNjYwMTYgMjIzLjAwMDAwMCBTIDEyNTAuMTY2MDE2IDIyMy4wMDAwMDAgMTI1MC4xNjYwMTYgMjMzLjAwMDAwMCBMIDEyNTAuMTY2MDE2IDk1OS4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0ibm9uZSIgY2xhc3M9ImNvbm5lY3Rpb24gc3Ryb2tlLUIxIiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiBtYXJrZXItZW5kPSJ1cmwoI21rLWQyLTQxMDI4NzYzMzktMzQ4ODM3ODEzNCkiIG1hc2s9InVybCgjZDItNDEwMjg3NjMzOSkiIC8+PHRleHQgeD0iMTI1MC4wMDAwMDAiIHk9IjQ0NC4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5nZW5lcmFsaXplZCBpbnRvPC90ZXh0PjwvZz48ZyBjbGFzcz0iS0dkNWJWOXNZWFZ1WTJndWJHVmhaRjl6WTNKaGJXSnNaWElnTFNabmREc2diR1ZoWkhNdVkyOXlaVjltYjNWeUtWc3dYUT09Ij48cGF0aCBkPSJNIDE3MDkuOTE2MDE2IDEzMC4wMDAwMDAgTCAxNzA5LjkxNjAxNiAzMTMuMDAwMDAwIFMgMTcwOS45MTYwMTYgMzIzLjAwMDAwMCAxNzE5LjkxNjAxNiAzMjMuMDAwMDAwIEwgMjE3Ny41MDAwMDAgMzIzLjAwMDAwMCBTIDIxODcuNTAwMDAwIDMyMy4wMDAwMDAgMjE4Ny41MDAwMDAgMzMzLjAwMDAwMCBMIDIxODcuNTAwMDAwIDIwNjYuMDAwMDAwIFMgMjE4Ny41MDAwMDAgMjA3Ni4wMDAwMDAgMjE3Ny41MDAwMDAgMjA3Ni4wMDAwMDAgTCAxNTgwLjU1NTA1NCAyMDc2LjAwMDAwMCBTIDE1NzAuNTU1MDU0IDIwNzYuMDAwMDAwIDE1NzAuNTU1MDU0IDIwODYuMDAwMDAwIEwgMTU3MC41NTUwNTQgMjE2Ny4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0ibm9uZSIgY2xhc3M9ImNvbm5lY3Rpb24gc3Ryb2tlLUIxIiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiBtYXJrZXItZW5kPSJ1cmwoI21rLWQyLTQxMDI4NzYzMzktMzQ4ODM3ODEzNCkiIG1hc2s9InVybCgjZDItNDEwMjg3NjMzOSkiIC8+PHRleHQgeD0iMjE4Ny41MDAwMDAiIHk9IjEyMjUuMDAwMDAwIiBmaWxsPSIjNjc2QzdFIiBjbGFzcz0idGV4dC1pdGFsaWMgZmlsbC1OMiIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+Zm9ybWFsaXplZCBpbnRvPC90ZXh0PjwvZz48ZyBjbGFzcz0iS0dkNWJWOXNZWFZ1WTJndWQyVmtaR2x1WjE5allXdGxJQzBtWjNRN0lHMXZibVY1WDIxdlpHVnNYMk52YlhCdmJtVnVkSE11WVhSMGNtRmpkR2x2YmlsYk1GMD0iPjxwYXRoIGQ9Ik0gMTk0NC45MTYwMTYgMTMwLjAwMDAwMCBMIDE5NDQuOTE2MDE2IDI2My4wMDAwMDAgUyAxOTQ0LjkxNjAxNiAyNzMuMDAwMDAwIDE5NTQuOTE2MDE2IDI3My4wMDAwMDAgTCAyNTQ5LjU4MzAwOCAyNzMuMDAwMDAwIFMgMjU1OS41ODMwMDggMjczLjAwMDAwMCAyNTU5LjU4MzAwOCAyODMuMDAwMDAwIEwgMjU1OS41ODMwMDggMzI4NC4wMDAwMDAgUyAyNTU5LjU4MzAwOCAzMjk0LjAwMDAwMCAyNTQ5LjU4MzAwOCAzMjk0LjAwMDAwMCBMIDE5MzQuMjYyOTM5IDMyOTQuMDAwMDAwIFMgMTkyNC4yNjI5MzkgMzI5NC4wMDAwMDAgMTkyNC4yNjI5MzkgMzMwNC4wMDAwMDAgTCAxOTI0LjI2MjkzOSAzMzg1LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSJub25lIiBjbGFzcz0iY29ubmVjdGlvbiBzdHJva2UtQjEiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNDg4Mzc4MTM0KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIyNTYwLjAwMDAwMCIgeT0iMTc3NC4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5hYnN0cmFjdGVkIGludG88L3RleHQ+PC9nPjxnIGNsYXNzPSJLR2Q1YlY5c1lYVnVZMmd1WTJaaElDMG1aM1E3SUdOaGMyaGZZM2xqYkdVcFd6QmQiPjxwYXRoIGQ9Ik0gMjQxNC45MTYwMTYgMTMwLjAwMDAwMCBMIDI0MTQuOTE2MDE2IDIxMy4wMDAwMDAgUyAyNDE0LjkxNjAxNiAyMjMuMDAwMDAwIDI0MjQuOTE2MDE2IDIyMy4wMDAwMDAgTCAyODMwLjA4MzAwOCAyMjMuMDAwMDAwIFMgMjg0MC4wODMwMDggMjIzLjAwMDAwMCAyODQwLjA4MzAwOCAyMzMuMDAwMDAwIEwgMjg0MC4wODMwMDggNDEwNS4wMDAwMDAgUyAyODQwLjA4MzAwOCA0MTE1LjAwMDAwMCAyODMwLjA4MzAwOCA0MTE1LjAwMDAwMCBMIDI1NDIuNzkxMDE2IDQxMTUuMDAwMDAwIFMgMjUzMi43OTEwMTYgNDExNS4wMDAwMDAgMjUzMi43OTEwMTYgNDEyNS4wMDAwMDAgTCAyNTMyLjc5MTAxNiA0MjAxLjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSJub25lIiBjbGFzcz0iY29ubmVjdGlvbiBzdHJva2UtQjEiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNDg4Mzc4MTM0KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIyODQwLjAwMDAwMCIgeT0iMjExMy4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5hYnN0cmFjdGVkIGludG88L3RleHQ+PC9nPjxnIGNsYXNzPSJiMlptWlhKelgyWnZkVzVrWVhScGIyNHVLRzFoY210bGRDQXRKbWQwT3lCaGRtRjBZWElwV3pCZCI+PHBhdGggZD0iTSAxMDgzLjgzMzAwOCA1NzcuMDAwMDAwIEwgMTA4My44MzMwMDggNzMyLjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSJub25lIiBjbGFzcz0iY29ubmVjdGlvbiBzdHJva2UtQjEiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNDg4Mzc4MTM0KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIxMDg0LjAwMDAwMCIgeT0iNjYxLjAwMDAwMCIgZmlsbD0iIzY3NkM3RSIgY2xhc3M9InRleHQtaXRhbGljIGZpbGwtTjIiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPnNlbGVjdHM8L3RleHQ+PC9nPjxnIGNsYXNzPSJiMlptWlhKelgyWnZkVzVrWVhScGIyNHVLR0YyWVhSaGNpQXRKbWQwT3lCMllXeDFaVjlsY1NsYk1GMD0iPjxwYXRoIGQ9Ik0gMTA4My44MzMwMDggODA0LjAwMDAwMCBMIDEwODMuODMzMDA4IDk1OS4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0ibm9uZSIgY2xhc3M9ImNvbm5lY3Rpb24gc3Ryb2tlLUIxIiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiBtYXJrZXItZW5kPSJ1cmwoI21rLWQyLTQxMDI4NzYzMzktMzQ4ODM3ODEzNCkiIG1hc2s9InVybCgjZDItNDEwMjg3NjMzOSkiIC8+PHRleHQgeD0iMTA4My41MDAwMDAiIHk9Ijg4OC4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5zY29yZXM8L3RleHQ+PC9nPjxnIGNsYXNzPSJLRzltWm1WeWMxOW1iM1Z1WkdGMGFXOXVMblpoYkhWbFgyVnhJQzBtWjNRN0lHZHpiMTlqYjIxd2IyNWxiblJ6TG1SbGJHbDJaWEo1WDJOMVltVXBXekJkIj48cGF0aCBkPSJNIDEwNDIuMjUwMDAwIDEwMzEuMDAwMDAwIEwgMTA0Mi4yNTAwMDAgMTExNC4wMDAwMDAgUyAxMDQyLjI1MDAwMCAxMTI0LjAwMDAwMCAxMDMyLjI1MDAwMCAxMTI0LjAwMDAwMCBMIDIwMS4wMDAwMDAgMTEyNC4wMDAwMDAgUyAxOTEuMDAwMDAwIDExMjQuMDAwMDAwIDE5MS4wMDAwMDAgMTEzNC4wMDAwMDAgTCAxOTEuMDAwMDAwIDEzNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjU0MS41MDAwMDAiIHk9IjExMzAuMDAwMDAwIiBmaWxsPSIjNjc2QzdFIiBjbGFzcz0idGV4dC1pdGFsaWMgZmlsbC1OMiIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+Z3VpZGVzPC90ZXh0PjwvZz48ZyBjbGFzcz0iS0c5bVptVnljMTltYjNWdVpHRjBhVzl1TG5aaGJIVmxYMlZ4SUMwbVozUTdJR2R6YjE5amIyMXdiMjVsYm5SekxuUnlhVzFmYzNSaFkyc3BXekJkIj48cGF0aCBkPSJNIDExNjcuMDAwMDAwIDEwMzEuMDAwMDAwIEwgMTE2Ny4wMDAwMDAgMTE2NC4wMDAwMDAgUyAxMTY3LjAwMDAwMCAxMTc0LjAwMDAwMCAxMTU3LjAwMDAwMCAxMTc0LjAwMDAwMCBMIDQ5Ni4wMDAwMDAgMTE3NC4wMDAwMDAgUyA0ODYuMDAwMDAwIDExNzQuMDAwMDAwIDQ4Ni4wMDAwMDAgMTE4NC4wMDAwMDAgTCA0ODYuMDAwMDAwIDEzNjYuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjgwMS41MDAwMDAiIHk9IjExODAuMDAwMDAwIiBmaWxsPSIjNjc2QzdFIiBjbGFzcz0idGV4dC1pdGFsaWMgZmlsbC1OMiIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+c2NvcmVzPC90ZXh0PjwvZz48ZyBjbGFzcz0iS0c5bVptVnljMTltYjNWdVpHRjBhVzl1TG5aaGJIVmxYMlZ4SUMwbVozUTdJR2R6YjE5amIyMXdiMjVsYm5SekxtSnZiblZ6WlhNcFd6QmQiPjxwYXRoIGQ9Ik0gMTI5MS43NTAwMDAgMTAzMS4wMDAwMDAgTCAxMjkxLjc1MDAwMCAxMjY1LjAwMDAwMCBTIDEyOTEuNzUwMDAwIDEyNzUuMDAwMDAwIDEyODEuNzUwMDAwIDEyNzUuMDAwMDAwIEwgNzU2LjUwMDAwMCAxMjc1LjAwMDAwMCBTIDc0Ni41MDAwMDAgMTI3NS4wMDAwMDAgNzQ2LjUwMDAwMCAxMjg1LjAwMDAwMCBMIDc0Ni41MDAwMDAgMTM2Ni4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0ibm9uZSIgY2xhc3M9ImNvbm5lY3Rpb24gc3Ryb2tlLUIxIiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiBtYXJrZXItZW5kPSJ1cmwoI21rLWQyLTQxMDI4NzYzMzktMzQ4ODM3ODEzNCkiIG1hc2s9InVybCgjZDItNDEwMjg3NjMzOSkiIC8+PHRleHQgeD0iMTA5NC41MDAwMDAiIHk9IjEyODEuMDAwMDAwIiBmaWxsPSIjNjc2QzdFIiBjbGFzcz0idGV4dC1pdGFsaWMgZmlsbC1OMiIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+c2NvcmVzPC90ZXh0PjwvZz48ZyBjbGFzcz0iS0dkemIxOWpiMjF3YjI1bGJuUnpMbVJsYkdsMlpYSjVYMk4xWW1VZ0xTWm5kRHNnWjNOdktWc3dYUT09Ij48cGF0aCBkPSJNIDE5MS4wMDAwMDAgMTQzOC4wMDAwMDAgTCAxOTEuMDAwMDAwIDE4MjIuMDAwMDAwIFMgMTkxLjAwMDAwMCAxODMyLjAwMDAwMCAyMDEuMDAwMDAwIDE4MzIuMDAwMDAwIEwgMTI2OS4wMjY5NzggMTgzMi4wMDAwMDAgUyAxMjc5LjAyNjk3OCAxODMyLjAwMDAwMCAxMjc5LjAyNjk3OCAxODQyLjAwMDAwMCBMIDEyNzkuMDI2OTc4IDE4NjguMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjU1Ny41MDAwMDAiIHk9IjE4MzguMDAwMDAwIiBmaWxsPSIjNjc2QzdFIiBjbGFzcz0idGV4dC1pdGFsaWMgZmlsbC1OMiIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+Y29uc3RydWN0czwvdGV4dD48L2c+PGcgY2xhc3M9IktHZHpiMTlqYjIxd2IyNWxiblJ6TG5SeWFXMWZjM1JoWTJzZ0xTWm5kRHNnWjNOdktWc3dYUT09Ij48cGF0aCBkPSJNIDQ4Ni4wMDAwMDAgMTQzOC4wMDAwMDAgTCA0ODYuMDAwMDAwIDE3NzIuMDAwMDAwIFMgNDg2LjAwMDAwMCAxNzgyLjAwMDAwMCA0OTYuMDAwMDAwIDE3ODIuMDAwMDAwIEwgMTMwNC41ODMwMDggMTc4Mi4wMDAwMDAgUyAxMzE0LjU4MzAwOCAxNzgyLjAwMDAwMCAxMzE0LjU4MzAwOCAxNzkyLjAwMDAwMCBMIDEzMTQuNTgzMDA4IDE4NjguMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9Ijc3Mi41MDAwMDAiIHk9IjE3ODguMDAwMDAwIiBmaWxsPSIjNjc2QzdFIiBjbGFzcz0idGV4dC1pdGFsaWMgZmlsbC1OMiIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+Y29uc3RydWN0czwvdGV4dD48L2c+PGcgY2xhc3M9IktHZHpiMTlqYjIxd2IyNWxiblJ6TG1KdmJuVnpaWE1nTFNabmREc2daM052S1Zzd1hRPT0iPjxwYXRoIGQ9Ik0gODAyLjc1MDAwMCAxNDM4LjAwMDAwMCBMIDgwMi43NTAwMDAgMTcyMi4wMDAwMDAgUyA4MDIuNzUwMDAwIDE3MzIuMDAwMDAwIDgxMi43NTAwMDAgMTczMi4wMDAwMDAgTCAxMzQwLjEzNzkzOSAxNzMyLjAwMDAwMCBTIDEzNTAuMTM3OTM5IDE3MzIuMDAwMDAwIDEzNTAuMTM3OTM5IDE3NDIuMDAwMDAwIEwgMTM1MC4xMzc5MzkgMTg2OC4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0ibm9uZSIgY2xhc3M9ImNvbm5lY3Rpb24gc3Ryb2tlLUIxIiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiBtYXJrZXItZW5kPSJ1cmwoI21rLWQyLTQxMDI4NzYzMzktMzQ4ODM3ODEzNCkiIG1hc2s9InVybCgjZDItNDEwMjg3NjMzOSkiIC8+PHRleHQgeD0iOTk4LjAwMDAwMCIgeT0iMTczOC4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5lbmhhbmNlczwvdGV4dD48L2c+PGcgY2xhc3M9IktHZHpiMTlqYjIxd2IyNWxiblJ6TG5OallYSmphWFI1SUMwbVozUTdJR2R6YnlsYk1GMD0iPjxwYXRoIGQ9Ik0gOTM0LjAwMDAwMCAxNDM4LjAwMDAwMCBMIDkzNC4wMDAwMDAgMTY3Mi4wMDAwMDAgUyA5MzQuMDAwMDAwIDE2ODIuMDAwMDAwIDk0NC4wMDAwMDAgMTY4Mi4wMDAwMDAgTCAxMzc1LjY5Mzk3MCAxNjgyLjAwMDAwMCBTIDEzODUuNjkzOTcwIDE2ODIuMDAwMDAwIDEzODUuNjkzOTcwIDE2OTIuMDAwMDAwIEwgMTM4NS42OTM5NzAgMTg2OC4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0ibm9uZSIgY2xhc3M9ImNvbm5lY3Rpb24gc3Ryb2tlLUIxIiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiBtYXJrZXItZW5kPSJ1cmwoI21rLWQyLTQxMDI4NzYzMzktMzQ4ODM3ODEzNCkiIG1hc2s9InVybCgjZDItNDEwMjg3NjMzOSkiIC8+PHRleHQgeD0iMTEzMi4wMDAwMDAiIHk9IjE2ODguMDAwMDAwIiBmaWxsPSIjNjc2QzdFIiBjbGFzcz0idGV4dC1pdGFsaWMgZmlsbC1OMiIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+ZW5oYW5jZXM8L3RleHQ+PC9nPjxnIGNsYXNzPSJLR2R6YjE5amIyMXdiMjVsYm5SekxuVnlaMlZ1WTNrZ0xTWm5kRHNnWjNOdktWc3dYUT09Ij48cGF0aCBkPSJNIDExMTMuNTAwMDAwIDE0MzguMDAwMDAwIEwgMTExMy41MDAwMDAgMTU3MS4wMDAwMDAgUyAxMTEzLjUwMDAwMCAxNTgxLjAwMDAwMCAxMTIzLjUwMDAwMCAxNTgxLjAwMDAwMCBMIDE0NjUuNjkzOTcwIDE1ODEuMDAwMDAwIFMgMTQ3NS42OTM5NzAgMTU4MS4wMDAwMDAgMTQ3NS42OTM5NzAgMTU5MS4wMDAwMDAgTCAxNDc1LjY5Mzk3MCAxNjcyLjAwMDAwMCBTIDE0NzUuNjkzOTcwIDE2ODIuMDAwMDAwIDE0NjUuNjkzOTcwIDE2ODIuMDAwMDAwIEwgMTQzMS4yNTAwMDAgMTY4Mi4wMDAwMDAgUyAxNDIxLjI1MDAwMCAxNjgyLjAwMDAwMCAxNDIxLjI1MDAwMCAxNjkyLjAwMDAwMCBMIDE0MjEuMjUwMDAwIDE4NjguMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjEzOTUuMDAwMDAwIiB5PSIxNTg3LjAwMDAwMCIgZmlsbD0iIzY3NkM3RSIgY2xhc3M9InRleHQtaXRhbGljIGZpbGwtTjIiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPmVuaGFuY2VzPC90ZXh0PjwvZz48ZyBjbGFzcz0iS0dkemIxOWpiMjF3YjI1bGJuUnpMbWQxWVhKaGJuUmxaWE1nTFNabmREc2daM052S1Zzd1hRPT0iPjxwYXRoIGQ9Ik0gMTU0NC4xOTM5NzAgMTQzOC4wMDAwMDAgTCAxNTQ0LjE5Mzk3MCAxNzIyLjAwMDAwMCBTIDE1NDQuMTkzOTcwIDE3MzIuMDAwMDAwIDE1MzQuMTkzOTcwIDE3MzIuMDAwMDAwIEwgMTQ2Ni44MDUwNTQgMTczMi4wMDAwMDAgUyAxNDU2LjgwNTA1NCAxNzMyLjAwMDAwMCAxNDU2LjgwNTA1NCAxNzQyLjAwMDAwMCBMIDE0NTYuODA1MDU0IDE4NjguMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjE1NDQuNTAwMDAwIiB5PSIxNzAzLjAwMDAwMCIgZmlsbD0iIzY3NkM3RSIgY2xhc3M9InRleHQtaXRhbGljIGZpbGwtTjIiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPnNlY3VyZXM8L3RleHQ+PC9nPjxnIGNsYXNzPSJLR2R6YjE5amIyMXdiMjVsYm5SekxtMWhaMmxqSUMwbVozUTdJR2R6YnlsYk1GMD0iPjxwYXRoIGQ9Ik0gMTg0My4wMDAwMDAgMTQzOC4wMDAwMDAgTCAxODQzLjAwMDAwMCAxNzcyLjAwMDAwMCBTIDE4NDMuMDAwMDAwIDE3ODIuMDAwMDAwIDE4MzMuMDAwMDAwIDE3ODIuMDAwMDAwIEwgMTUwMi4zNjA5NjIgMTc4Mi4wMDAwMDAgUyAxNDkyLjM2MDk2MiAxNzgyLjAwMDAwMCAxNDkyLjM2MDk2MiAxNzkyLjAwMDAwMCBMIDE0OTIuMzYwOTYyIDE4NjguMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjE3OTUuNTAwMDAwIiB5PSIxNzg4LjAwMDAwMCIgZmlsbD0iIzY3NkM3RSIgY2xhc3M9InRleHQtaXRhbGljIGZpbGwtTjIiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPm5hbWVzPC90ZXh0PjwvZz48ZyBjbGFzcz0iS0dkemJ5QXRKbWQwT3lCc1pXRmtjeTVqYjNKbFgyWnZkWElwV3pCZCI+PHBhdGggZD0iTSAxNDQyLjg4NzkzOSAxOTQ3LjAwMDAwMCBMIDE0NDIuODg3OTM5IDIxNjcuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjE0NDIuNTAwMDAwIiB5PSIyMDY0LjAwMDAwMCIgZmlsbD0iIzY3NkM3RSIgY2xhc3M9InRleHQtaXRhbGljIGZpbGwtTjIiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPlBSRVJFUVVJU0lURSAtIChubyBsZWFkcyB3aXRob3V0IGNvbnZlcnRpbmcgb2ZmZXIpPC90ZXh0PjwvZz48ZyBjbGFzcz0iYkdWaFpITXVLR052Y21WZlptOTFjaUF0Sm1kME95QnNaV0ZrWDIxaFoyNWxkQ2xiTUYwPSI+PHBhdGggZD0iTSAxNDY4LjQyMTk5NyAyMjM5LjAwMDAwMCBMIDE0NjguNDIxOTk3IDIzMTcuMDAwMDAwIFMgMTQ2OC40MjE5OTcgMjMyNy4wMDAwMDAgMTQ1OC40MjE5OTcgMjMyNy4wMDAwMDAgTCAxMzU5Ljk3MjA0NiAyMzI3LjAwMDAwMCBTIDEzNDkuOTcyMDQ2IDIzMjcuMDAwMDAwIDEzNDkuOTcyMDQ2IDIzMzcuMDAwMDAwIEwgMTM0OS45NzIwNDYgMjQ1NC4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0ibm9uZSIgY2xhc3M9ImNvbm5lY3Rpb24gc3Ryb2tlLUIxIiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiBtYXJrZXItZW5kPSJ1cmwoI21rLWQyLTQxMDI4NzYzMzktMzQ4ODM3ODEzNCkiIG1hc2s9InVybCgjZDItNDEwMjg3NjMzOSkiIC8+PHRleHQgeD0iMTM4OC41MDAwMDAiIHk9IjIzMzMuMDAwMDAwIiBmaWxsPSIjNjc2QzdFIiBjbGFzcz0idGV4dC1pdGFsaWMgZmlsbC1OMiIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+Y2hhbm5lbHMgbmVlZDwvdGV4dD48L2c+PGcgY2xhc3M9ImJHVmhaSE11S0d4bFlXUmZiV0ZuYm1WMElDMG1aM1E3SUhKMWJHVmZNVEF3S1Zzd1hRPT0iPjxwYXRoIGQ9Ik0gMTQwMC45NzIwNDYgMjUyNi4wMDAwMDAgTCAxNDAwLjk3MjA0NiAyNjkxLjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSJub25lIiBjbGFzcz0iY29ubmVjdGlvbiBzdHJva2UtQjEiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNDg4Mzc4MTM0KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIxNDAwLjUwMDAwMCIgeT0iMjYxNS4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5mdWVsPC90ZXh0PjwvZz48ZyBjbGFzcz0iYkdWaFpITXVLR052Y21WZlptOTFjaUF0Sm1kME95QnNaV0ZrWDJkbGRIUmxjbk1wV3pCZCI+PHBhdGggZD0iTSAxNTQ1LjAyMTk3MyAyMjM5LjAwMDAwMCBMIDE1NDUuMDIxOTczIDI0NTQuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjE1NDUuMDAwMDAwIiB5PSIyMzUzLjAwMDAwMCIgZmlsbD0iIzY3NkM3RSIgY2xhc3M9InRleHQtaXRhbGljIGZpbGwtTjIiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPmRlbGVnYXRlZCB2aWE8L3RleHQ+PC9nPjxnIGNsYXNzPSJiR1ZoWkhNdUtHTnZjbVZmWm05MWNpQXRKbWQwT3lCeVpXWmxjbkpoYkhNcFd6QmQiPjxwYXRoIGQ9Ik0gMTYyMS42MjE5NDggMjIzOS4wMDAwMDAgTCAxNjIxLjYyMTk0OCAyMjY3LjAwMDAwMCBTIDE2MjEuNjIxOTQ4IDIyNzcuMDAwMDAwIDE2MzEuNjIxOTQ4IDIyNzcuMDAwMDAwIEwgMTk0Ni40NzIwNDYgMjI3Ny4wMDAwMDAgUyAxOTU2LjQ3MjA0NiAyMjc3LjAwMDAwMCAxOTU2LjQ3MjA0NiAyMjg3LjAwMDAwMCBMIDE5NTYuNDcyMDQ2IDI0NTQuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjE4NTkuNTAwMDAwIiB5PSIyMjgzLjAwMDAwMCIgZmlsbD0iIzY3NkM3RSIgY2xhc3M9InRleHQtaXRhbGljIGZpbGwtTjIiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPmFjdGl2YXRlczwvdGV4dD48L2c+PGcgY2xhc3M9ImJHVmhaSE11S0hKbFptVnljbUZzY3lBdEptZDBPeUJ4ZFdGc2FXWnBaWElwV3pCZCI+PHBhdGggZD0iTSAyMDI5LjIyMjA0NiAyNTI2LjAwMDAwMCBMIDIwMjkuMjIyMDQ2IDI2OTEuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjIwMjkuMDAwMDAwIiB5PSIyNjE1LjAwMDAwMCIgZmlsbD0iIzY3NkM3RSIgY2xhc3M9InRleHQtaXRhbGljIGZpbGwtTjIiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPnVzZXM8L3RleHQ+PC9nPjxnIGNsYXNzPSJLR3hsWVdSekxtTnZjbVZmWm05MWNpQXRKbWQwT3lCc1pXRmtLVnN3WFE9PSI+PHBhdGggZD0iTSAxMzkxLjgyMjAyMSAyMjM5LjAwMDAwMCBMIDEzOTEuODIyMDIxIDIyNjcuMDAwMDAwIFMgMTM5MS44MjIwMjEgMjI3Ny4wMDAwMDAgMTM4MS44MjIwMjEgMjI3Ny4wMDAwMDAgTCAxMjA4Ljk3MjA0NiAyMjc3LjAwMDAwMCBTIDExOTguOTcyMDQ2IDIyNzcuMDAwMDAwIDExOTguOTcyMDQ2IDIyODcuMDAwMDAwIEwgMTE5OC45NzIwNDYgMzA0Ny4wMDAwMDAgUyAxMTk4Ljk3MjA0NiAzMDU3LjAwMDAwMCAxMjA4Ljk3MjA0NiAzMDU3LjAwMDAwMCBMIDE2MTIuNDMwMDU0IDMwNTcuMDAwMDAwIFMgMTYyMi40MzAwNTQgMzA1Ny4wMDAwMDAgMTYyMi40MzAwNTQgMzA2Ny4wMDAwMDAgTCAxNjIyLjQzMDA1NCAzMDkzLjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSJub25lIiBjbGFzcz0iY29ubmVjdGlvbiBzdHJva2UtQjEiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNDg4Mzc4MTM0KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIxMTk5LjAwMDAwMCIgeT0iMjc4OC4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5wcm9kdWNlczwvdGV4dD48L2c+PGcgY2xhc3M9IktHeGxZV1J6TG14bFlXUmZiV0ZuYm1WMElDMG1aM1E3SUd4bFlXUXBXekJkIj48cGF0aCBkPSJNIDEyOTguOTcyMDQ2IDI1MjYuMDAwMDAwIEwgMTI5OC45NzIwNDYgMjk5Ny4wMDAwMDAgUyAxMjk4Ljk3MjA0NiAzMDA3LjAwMDAwMCAxMzA4Ljk3MjA0NiAzMDA3LjAwMDAwMCBMIDE2NDkuNTk3MDQ2IDMwMDcuMDAwMDAwIFMgMTY1OS41OTcwNDYgMzAwNy4wMDAwMDAgMTY1OS41OTcwNDYgMzAxNy4wMDAwMDAgTCAxNjU5LjU5NzA0NiAzMDkzLjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSJub25lIiBjbGFzcz0iY29ubmVjdGlvbiBzdHJva2UtQjEiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNDg4Mzc4MTM0KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIxMjk5LjAwMDAwMCIgeT0iMjk5Ni4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5wcm9kdWNlczwvdGV4dD48L2c+PGcgY2xhc3M9IktHeGxZV1J6TG5KMWJHVmZNVEF3SUMwbVozUTdJR3hsWVdRcFd6QmQiPjxwYXRoIGQ9Ik0gMTQwMC45NzIwNDYgMjc2My4wMDAwMDAgTCAxNDAwLjk3MjA0NiAyOTQ3LjAwMDAwMCBTIDE0MDAuOTcyMDQ2IDI5NTcuMDAwMDAwIDE0MTAuOTcyMDQ2IDI5NTcuMDAwMDAwIEwgMTY4Ni43NjI5MzkgMjk1Ny4wMDAwMDAgUyAxNjk2Ljc2MjkzOSAyOTU3LjAwMDAwMCAxNjk2Ljc2MjkzOSAyOTY3LjAwMDAwMCBMIDE2OTYuNzYyOTM5IDMwOTMuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjE1MjEuMDAwMDAwIiB5PSIyOTYzLjAwMDAwMCIgZmlsbD0iIzY3NkM3RSIgY2xhc3M9InRleHQtaXRhbGljIGZpbGwtTjIiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPmNvbXBvdW5kczwvdGV4dD48L2c+PGcgY2xhc3M9IktHeGxZV1J6TG14bFlXUmZaMlYwZEdWeWN5QXRKbWQwT3lCc1pXRmtLVnN3WFE9PSI+PHBhdGggZD0iTSAxNzc3Ljc2MjkzOSAyNTI2LjAwMDAwMCBMIDE3NzcuNzYyOTM5IDI5NDcuMDAwMDAwIFMgMTc3Ny43NjI5MzkgMjk1Ny4wMDAwMDAgMTc2Ny43NjI5MzkgMjk1Ny4wMDAwMDAgTCAxNzQzLjkzMDA1NCAyOTU3LjAwMDAwMCBTIDE3MzMuOTMwMDU0IDI5NTcuMDAwMDAwIDE3MzMuOTMwMDU0IDI5NjcuMDAwMDAwIEwgMTczMy45MzAwNTQgMzA5My4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0ibm9uZSIgY2xhc3M9ImNvbm5lY3Rpb24gc3Ryb2tlLUIxIiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiBtYXJrZXItZW5kPSJ1cmwoI21rLWQyLTQxMDI4NzYzMzktMzQ4ODM3ODEzNCkiIG1hc2s9InVybCgjZDItNDEwMjg3NjMzOSkiIC8+PHRleHQgeD0iMTc3OC4wMDAwMDAiIHk9IjI4MzguMDAwMDAwIiBmaWxsPSIjNjc2QzdFIiBjbGFzcz0idGV4dC1pdGFsaWMgZmlsbC1OMiIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+cHJvZHVjZXM8L3RleHQ+PC9nPjxnIGNsYXNzPSJLR3hsWVdSekxuSmxabVZ5Y21Gc2N5QXRKbWQwT3lCc1pXRmtLVnN3WFE9PSI+PHBhdGggZD0iTSAxOTI2LjYzNzkzOSAyNTI2LjAwMDAwMCBMIDE5MjYuNjM3OTM5IDI1NTQuMDAwMDAwIFMgMTkyNi42Mzc5MzkgMjU2NC4wMDAwMDAgMTkxNi42Mzc5MzkgMjU2NC4wMDAwMDAgTCAxODYwLjc2MjkzOSAyNTY0LjAwMDAwMCBTIDE4NTAuNzYyOTM5IDI1NjQuMDAwMDAwIDE4NTAuNzYyOTM5IDI1NzQuMDAwMDAwIEwgMTg1MC43NjI5MzkgMjk5Ny4wMDAwMDAgUyAxODUwLjc2MjkzOSAzMDA3LjAwMDAwMCAxODQwLjc2MjkzOSAzMDA3LjAwMDAwMCBMIDE3ODEuMDk3MDQ2IDMwMDcuMDAwMDAwIFMgMTc3MS4wOTcwNDYgMzAwNy4wMDAwMDAgMTc3MS4wOTcwNDYgMzAxNy4wMDAwMDAgTCAxNzcxLjA5NzA0NiAzMDkzLjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSJub25lIiBjbGFzcz0iY29ubmVjdGlvbiBzdHJva2UtQjEiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNDg4Mzc4MTM0KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIxODUxLjAwMDAwMCIgeT0iMjgxOC4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5wcm9kdWNlczwvdGV4dD48L2c+PGcgY2xhc3M9IktHeGxZV1FnTFNabmREc2diVzl1WlhsZmJXOWtaV3hmWTI5dGNHOXVaVzUwY3k1aGRIUnlZV04wYVc5dUtWc3dYUT09Ij48cGF0aCBkPSJNIDE2OTYuNzYyOTM5IDMxNjUuMDAwMDAwIEwgMTY5Ni43NjI5MzkgMzI4NC4wMDAwMDAgUyAxNjk2Ljc2MjkzOSAzMjk0LjAwMDAwMCAxNzA2Ljc2MjkzOSAzMjk0LjAwMDAwMCBMIDE4MTYuMjYyOTM5IDMyOTQuMDAwMDAwIFMgMTgyNi4yNjI5MzkgMzI5NC4wMDAwMDAgMTgyNi4yNjI5MzkgMzMwNC4wMDAwMDAgTCAxODI2LjI2MjkzOSAzMzg1LjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSJub25lIiBjbGFzcz0iY29ubmVjdGlvbiBzdHJva2UtQjEiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNDg4Mzc4MTM0KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIxNzQzLjUwMDAwMCIgeT0iMzMwMC4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5lbnRlcnMgdmlhPC90ZXh0PjwvZz48ZyBjbGFzcz0iYlc5dVpYbGZiVzlrWld4ZlkyOXRjRzl1Wlc1MGN5NG9ZWFIwY21GamRHbHZiaUF0Sm1kME95QjFjSE5sYkd3cFd6QmQiPjxwYXRoIGQ9Ik0gMTc1Mi41MTI5MzkgMzQ1Ny4wMDAwMDAgTCAxNzUyLjUxMjkzOSAzNjIyLjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSJub25lIiBjbGFzcz0iY29ubmVjdGlvbiBzdHJva2UtQjEiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNDg4Mzc4MTM0KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIxNzUzLjAwMDAwMCIgeT0iMzU0Ni4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5pZiB5ZXM8L3RleHQ+PC9nPjxnIGNsYXNzPSJiVzl1WlhsZmJXOWtaV3hmWTI5dGNHOXVaVzUwY3k0b1lYUjBjbUZqZEdsdmJpQXRKbWQwT3lCa2IzZHVjMlZzYkNsYk1GMD0iPjxwYXRoIGQ9Ik0gMTk5OC4wMTI5MzkgMzQ1Ny4wMDAwMDAgTCAxOTk4LjAxMjkzOSAzNjIyLjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSJub25lIiBjbGFzcz0iY29ubmVjdGlvbiBzdHJva2UtQjEiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNDg4Mzc4MTM0KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIxOTk4LjAwMDAwMCIgeT0iMzU0Ni4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5pZiBubzwvdGV4dD48L2c+PGcgY2xhc3M9ImJXOXVaWGxmYlc5a1pXeGZZMjl0Y0c5dVpXNTBjeTRvZFhCelpXeHNJQzBtWjNRN0lHTnZiblJwYm5WcGRIa3BXekJkIj48cGF0aCBkPSJNIDE4MzMuNDMwMDU0IDM2OTQuMDAwMDAwIEwgMTgzMy40MzAwNTQgMzg1OS4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0ibm9uZSIgY2xhc3M9ImNvbm5lY3Rpb24gc3Ryb2tlLUIxIiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiBtYXJrZXItZW5kPSJ1cmwoI21rLWQyLTQxMDI4NzYzMzktMzQ4ODM3ODEzNCkiIG1hc2s9InVybCgjZDItNDEwMjg3NjMzOSkiIC8+PHRleHQgeD0iMTgzMy4wMDAwMDAiIHk9IjM3ODMuMDAwMDAwIiBmaWxsPSIjNjc2QzdFIiBjbGFzcz0idGV4dC1pdGFsaWMgZmlsbC1OMiIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+dGhlbjwvdGV4dD48L2c+PGcgY2xhc3M9ImJXOXVaWGxmYlc5a1pXeGZZMjl0Y0c5dVpXNTBjeTRvWkc5M2JuTmxiR3dnTFNabmREc2dZMjl1ZEdsdWRXbDBlU2xiTUYwPSI+PHBhdGggZD0iTSAxOTE3LjA5NzA0NiAzNjk0LjAwMDAwMCBMIDE5MTcuMDk3MDQ2IDM4NTkuMDAwMDAwIiBzdHJva2U9IiMwRDMyQjIiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIHN0cm9rZS1CMSIgc3R5bGU9InN0cm9rZS13aWR0aDoyOyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM0ODgzNzgxMzQpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjE5MTcuMDAwMDAwIiB5PSIzNzgzLjAwMDAwMCIgZmlsbD0iIzY3NkM3RSIgY2xhc3M9InRleHQtaXRhbGljIGZpbGwtTjIiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPnRoZW48L3RleHQ+PC9nPjxnIGNsYXNzPSJLRzF2Ym1WNVgyMXZaR1ZzWDJOdmJYQnZibVZ1ZEhNdVkyOXVkR2x1ZFdsMGVTQXRKbWQwT3lCallYTm9YMk41WTJ4bEtWc3dYUT09Ij48cGF0aCBkPSJNIDE4NzUuMjYyOTM5IDM5MzEuMDAwMDAwIEwgMTg3NS4yNjI5MzkgNDEwNS4wMDAwMDAgUyAxODc1LjI2MjkzOSA0MTE1LjAwMDAwMCAxODg1LjI2MjkzOSA0MTE1LjAwMDAwMCBMIDI0MjEuNzkxMDE2IDQxMTUuMDAwMDAwIFMgMjQzMS43OTEwMTYgNDExNS4wMDAwMDAgMjQzMS43OTEwMTYgNDEyNS4wMDAwMDAgTCAyNDMxLjc5MTAxNiA0MjAxLjAwMDAwMCIgc3Ryb2tlPSIjMEQzMkIyIiBmaWxsPSJub25lIiBjbGFzcz0iY29ubmVjdGlvbiBzdHJva2UtQjEiIHN0eWxlPSJzdHJva2Utd2lkdGg6MjsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNDg4Mzc4MTM0KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIyMTA1LjUwMDAwMCIgeT0iNDEyMS4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5mdW5kczwvdGV4dD48L2c+PGcgY2xhc3M9IktHTmhjMmhmWTNsamJHVWdMU1puZERzZ2MyTmhiR1VwV3pCZCI+PHBhdGggZD0iTSAyNTMyLjc5MTAxNiA0Mjc4LjAwMDAwMCBMIDI1MzIuNzkxMDE2IDQzOTcuMDAwMDAwIFMgMjUzMi43OTEwMTYgNDQwNy4wMDAwMDAgMjU0Mi43OTEwMTYgNDQwNy4wMDAwMDAgTCAyODg4LjU0MTAxNiA0NDA3LjAwMDAwMCBTIDI4OTguNTQxMDE2IDQ0MDcuMDAwMDAwIDI4OTguNTQxMDE2IDQ0MTcuMDAwMDAwIEwgMjg5OC41NDEwMTYgNDQ5My4wMDAwMDAiIHN0cm9rZT0iIzBEMzJCMiIgZmlsbD0ibm9uZSIgY2xhc3M9ImNvbm5lY3Rpb24gc3Ryb2tlLUIxIiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7IiBtYXJrZXItZW5kPSJ1cmwoI21rLWQyLTQxMDI4NzYzMzktMzQ4ODM3ODEzNCkiIG1hc2s9InVybCgjZDItNDEwMjg3NjMzOSkiIC8+PHRleHQgeD0iMjY5NS4wMDAwMDAiIHk9IjQ0MTMuMDAwMDAwIiBmaWxsPSIjNjc2QzdFIiBjbGFzcz0idGV4dC1pdGFsaWMgZmlsbC1OMiIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+aWYgcG9zaXRpdmUsIHNjYWxlPC90ZXh0PjwvZz48ZyBjbGFzcz0iS0d4dmMzUmZZMmhoY0hSbGNuTXVjR1Z5Wm05eWJXRnVZMlZmWkdsaGJXOXVaQ0F0Sm1kME95Qm5jMjhwV3pCZCI+PG1hcmtlciBpZD0ibWstZDItNDEwMjg3NjMzOS0zNzQzNDQzNjM2IiBtYXJrZXJXaWR0aD0iMTAuMDAwMDAwIiBtYXJrZXJIZWlnaHQ9IjEyLjAwMDAwMCIgcmVmWD0iNy4wMDAwMDAiIHJlZlk9IjYuMDAwMDAwIiB2aWV3Qm94PSIwLjAwMDAwMCAwLjAwMDAwMCAxMC4wMDAwMDAgMTIuMDAwMDAwIiBvcmllbnQ9ImF1dG8iIG1hcmtlclVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+IDxwb2x5Z29uIHBvaW50cz0iMC4wMDAwMDAsMC4wMDAwMDAgMTAuMDAwMDAwLDYuMDAwMDAwIDAuMDAwMDAwLDEyLjAwMDAwMCIgZmlsbD0iIzZiMjFhOCIgY2xhc3M9ImNvbm5lY3Rpb24iIHN0cm9rZS13aWR0aD0iMiIgLz4gPC9tYXJrZXI+PHBhdGggZD0iTSAzMDE4LjMzMzAwOCAxNDM4LjAwMDAwMCBMIDMwMTguMzMzMDA4IDE4MjIuMDAwMDAwIFMgMzAxOC4zMzMwMDggMTgzMi4wMDAwMDAgMzAwOC4zMzMwMDggMTgzMi4wMDAwMDAgTCAxNTM3LjkxNjAxNiAxODMyLjAwMDAwMCBTIDE1MjcuOTE2MDE2IDE4MzIuMDAwMDAwIDE1MjcuOTE2MDE2IDE4NDIuMDAwMDAwIEwgMTUyNy45MTYwMTYgMTg2OC4wMDAwMDAiIHN0cm9rZT0iIzZiMjFhOCIgZmlsbD0ibm9uZSIgY2xhc3M9ImNvbm5lY3Rpb24iIHN0eWxlPSJzdHJva2Utd2lkdGg6MjtzdHJva2UtZGFzaGFycmF5OjEwLjAwMDAwMCw5Ljg2NTYzOTsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNzQzNDQzNjM2KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIyNDUxLjAwMDAwMCIgeT0iMTgzOC4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij5kaWFnbm9zZXM8L3RleHQ+PC9nPjxnIGNsYXNzPSJLR3h2YzNSZlkyaGhjSFJsY25NdWJIUm5jRjl0WVhSb0lDMG1aM1E3SUdOaGMyaGZZM2xqYkdVcFd6QmQiPjxwYXRoIGQ9Ik0gMzQyNS44MzMwMDggMTQzOC4wMDAwMDAgTCAzNDI1LjgzMzAwOCA0MTU1LjAwMDAwMCBTIDM0MjUuODMzMDA4IDQxNjUuMDAwMDAwIDM0MTUuODMzMDA4IDQxNjUuMDAwMDAwIEwgMjY0My43OTEwMTYgNDE2NS4wMDAwMDAgUyAyNjMzLjc5MTAxNiA0MTY1LjAwMDAwMCAyNjMzLjc5MTAxNiA0MTc1LjAwMDAwMCBMIDI2MzMuNzkxMDE2IDQyMDEuMDAwMDAwIiBzdHJva2U9IiM2YjIxYTgiIGZpbGw9Im5vbmUiIGNsYXNzPSJjb25uZWN0aW9uIiBzdHlsZT0ic3Ryb2tlLXdpZHRoOjI7c3Ryb2tlLWRhc2hhcnJheToxMC4wMDAwMDAsOS44NjU2Mzk7IiBtYXJrZXItZW5kPSJ1cmwoI21rLWQyLTQxMDI4NzYzMzktMzc0MzQ0MzYzNikiIG1hc2s9InVybCgjZDItNDEwMjg3NjMzOSkiIC8+PHRleHQgeD0iMzQyNS41MDAwMDAiIHk9IjMyMjIuMDAwMDAwIiBmaWxsPSIjNjc2QzdFIiBjbGFzcz0idGV4dC1pdGFsaWMgZmlsbC1OMiIgc3R5bGU9InRleHQtYW5jaG9yOm1pZGRsZTtmb250LXNpemU6MTZweCI+bWVhc3VyZXM8L3RleHQ+PC9nPjxnIGNsYXNzPSJLR3h2YzNSZlkyaGhjSFJsY25NdWJIUm5jRjl0WVhSb0lDMG1aM1E3SUhOallXeGxLVnN3WFE9PSI+PHBhdGggZD0iTSAzNDk5LjMzMzAwOCAxNDM4LjAwMDAwMCBMIDM0OTkuMzMzMDA4IDQzOTcuMDAwMDAwIFMgMzQ5OS4zMzMwMDggNDQwNy4wMDAwMDAgMzQ4OS4zMzMwMDggNDQwNy4wMDAwMDAgTCAyOTg2Ljc5MTAxNiA0NDA3LjAwMDAwMCBTIDI5NzYuNzkxMDE2IDQ0MDcuMDAwMDAwIDI5NzYuNzkxMDE2IDQ0MTcuMDAwMDAwIEwgMjk3Ni43OTEwMTYgNDQ5My4wMDAwMDAiIHN0cm9rZT0iIzZiMjFhOCIgZmlsbD0ibm9uZSIgY2xhc3M9ImNvbm5lY3Rpb24iIHN0eWxlPSJzdHJva2Utd2lkdGg6MjtzdHJva2UtZGFzaGFycmF5OjEwLjAwMDAwMCw5Ljg2NTYzOTsiIG1hcmtlci1lbmQ9InVybCgjbWstZDItNDEwMjg3NjMzOS0zNzQzNDQzNjM2KSIgbWFzaz0idXJsKCNkMi00MTAyODc2MzM5KSIgLz48dGV4dCB4PSIzNDk5LjAwMDAwMCIgeT0iMzIzMy4wMDAwMDAiIGZpbGw9IiM2NzZDN0UiIGNsYXNzPSJ0ZXh0LWl0YWxpYyBmaWxsLU4yIiBzdHlsZT0idGV4dC1hbmNob3I6bWlkZGxlO2ZvbnQtc2l6ZToxNnB4Ij50cmlnZ2VyczwvdGV4dD48L2c+PGcgY2xhc3M9IktHeHZjM1JmWTJoaGNIUmxjbk11WkhOZmMzbHpkR1Z0SUMwbVozUTdJSE5qWVd4bEtWc3dYUT09Ij48cGF0aCBkPSJNIDM2NzIuNTgzMDA4IDE0MzguMDAwMDAwIEwgMzY3Mi41ODMwMDggNDQ0Ny4wMDAwMDAgUyAzNjcyLjU4MzAwOCA0NDU3LjAwMDAwMCAzNjYyLjU4MzAwOCA0NDU3LjAwMDAwMCBMIDMwNjUuMDQxMDE2IDQ0NTcuMDAwMDAwIFMgMzA1NS4wNDEwMTYgNDQ1Ny4wMDAwMDAgMzA1NS4wNDEwMTYgNDQ2Ny4wMDAwMDAgTCAzMDU1LjA0MTAxNiA0NDkzLjAwMDAwMCIgc3Ryb2tlPSIjNmIyMWE4IiBmaWxsPSJub25lIiBjbGFzcz0iY29ubmVjdGlvbiIgc3R5bGU9InN0cm9rZS13aWR0aDoyO3N0cm9rZS1kYXNoYXJyYXk6MTAuMDAwMDAwLDkuODY1NjM5OyIgbWFya2VyLWVuZD0idXJsKCNtay1kMi00MTAyODc2MzM5LTM3NDM0NDM2MzYpIiBtYXNrPSJ1cmwoI2QyLTQxMDI4NzYzMzkpIiAvPjx0ZXh0IHg9IjM2NzMuMDAwMDAwIiB5PSIzMjgxLjAwMDAwMCIgZmlsbD0iIzY3NkM3RSIgY2xhc3M9InRleHQtaXRhbGljIGZpbGwtTjIiIHN0eWxlPSJ0ZXh0LWFuY2hvcjptaWRkbGU7Zm9udC1zaXplOjE2cHgiPmVuYWJsZXM8L3RleHQ+PC9nPjxtYXNrIGlkPSJkMi00MTAyODc2MzM5IiBtYXNrVW5pdHM9InVzZXJTcGFjZU9uVXNlIiB4PSItODkiIHk9Ii04OSIgd2lkdGg9IjQ3NTAiIGhlaWdodD0iNDc2MCI+CjxyZWN0IHg9Ii04OSIgeT0iLTg5IiB3aWR0aD0iNDc1MCIgaGVpZ2h0PSI0NzYwIiBmaWxsPSJ3aGl0ZSI+PC9yZWN0Pgo8cmVjdCB4PSIxMTk1LjAwMDAwMCIgeT0iNDI4LjAwMDAwMCIgd2lkdGg9IjExMCIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iMjEzNi4wMDAwMDAiIHk9IjEyMDkuMDAwMDAwIiB3aWR0aD0iMTAzIiBoZWlnaHQ9IjIxIiBmaWxsPSJibGFjayI+PC9yZWN0Pgo8cmVjdCB4PSIyNTA4LjAwMDAwMCIgeT0iMTc1OC4wMDAwMDAiIHdpZHRoPSIxMDQiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjI3ODguMDAwMDAwIiB5PSIyMDk3LjAwMDAwMCIgd2lkdGg9IjEwNCIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iMTA1OS4wMDAwMDAiIHk9IjY0NS4wMDAwMDAiIHdpZHRoPSI1MCIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iMTA2MC4wMDAwMDAiIHk9Ijg3Mi4wMDAwMDAiIHdpZHRoPSI0NyIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iNTE3LjAwMDAwMCIgeT0iMTExNC4wMDAwMDAiIHdpZHRoPSI0OSIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iNzc4LjAwMDAwMCIgeT0iMTE2NC4wMDAwMDAiIHdpZHRoPSI0NyIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iMTA3MS4wMDAwMDAiIHk9IjEyNjUuMDAwMDAwIiB3aWR0aD0iNDciIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjUyMS4wMDAwMDAiIHk9IjE4MjIuMDAwMDAwIiB3aWR0aD0iNzMiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjczNi4wMDAwMDAiIHk9IjE3NzIuMDAwMDAwIiB3aWR0aD0iNzMiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9Ijk2NC4wMDAwMDAiIHk9IjE3MjIuMDAwMDAwIiB3aWR0aD0iNjgiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjEwOTguMDAwMDAwIiB5PSIxNjcyLjAwMDAwMCIgd2lkdGg9IjY4IiBoZWlnaHQ9IjIxIiBmaWxsPSJibGFjayI+PC9yZWN0Pgo8cmVjdCB4PSIxMzYxLjAwMDAwMCIgeT0iMTU3MS4wMDAwMDAiIHdpZHRoPSI2OCIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iMTUxNy4wMDAwMDAiIHk9IjE2ODcuMDAwMDAwIiB3aWR0aD0iNTUiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjE3NzEuMDAwMDAwIiB5PSIxNzcyLjAwMDAwMCIgd2lkdGg9IjQ5IiBoZWlnaHQ9IjIxIiBmaWxsPSJibGFjayI+PC9yZWN0Pgo8cmVjdCB4PSIxMjc1LjAwMDAwMCIgeT0iMjA0OC4wMDAwMDAiIHdpZHRoPSIzMzUiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjEzMzkuMDAwMDAwIiB5PSIyMzE3LjAwMDAwMCIgd2lkdGg9Ijk5IiBoZWlnaHQ9IjIxIiBmaWxsPSJibGFjayI+PC9yZWN0Pgo8cmVjdCB4PSIxMzg2LjAwMDAwMCIgeT0iMjU5OS4wMDAwMDAiIHdpZHRoPSIyOSIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iMTQ5OC4wMDAwMDAiIHk9IjIzMzcuMDAwMDAwIiB3aWR0aD0iOTQiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjE4MjcuMDAwMDAwIiB5PSIyMjY3LjAwMDAwMCIgd2lkdGg9IjY1IiBoZWlnaHQ9IjIxIiBmaWxsPSJibGFjayI+PC9yZWN0Pgo8cmVjdCB4PSIyMDEyLjAwMDAwMCIgeT0iMjU5OS4wMDAwMDAiIHdpZHRoPSIzNCIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iMTE2Ni4wMDAwMDAiIHk9IjI3NzIuMDAwMDAwIiB3aWR0aD0iNjYiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjEyNjYuMDAwMDAwIiB5PSIyOTgwLjAwMDAwMCIgd2lkdGg9IjY2IiBoZWlnaHQ9IjIxIiBmaWxsPSJibGFjayI+PC9yZWN0Pgo8cmVjdCB4PSIxNDgwLjAwMDAwMCIgeT0iMjk0Ny4wMDAwMDAiIHdpZHRoPSI4MiIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iMTc0NS4wMDAwMDAiIHk9IjI4MjIuMDAwMDAwIiB3aWR0aD0iNjYiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjE4MTguMDAwMDAwIiB5PSIyODAyLjAwMDAwMCIgd2lkdGg9IjY2IiBoZWlnaHQ9IjIxIiBmaWxsPSJibGFjayI+PC9yZWN0Pgo8cmVjdCB4PSIxNzA5LjAwMDAwMCIgeT0iMzI4NC4wMDAwMDAiIHdpZHRoPSI2OSIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iMTczNC4wMDAwMDAiIHk9IjM1MzAuMDAwMDAwIiB3aWR0aD0iMzgiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjE5ODIuMDAwMDAwIiB5PSIzNTMwLjAwMDAwMCIgd2lkdGg9IjMyIiBoZWlnaHQ9IjIxIiBmaWxsPSJibGFjayI+PC9yZWN0Pgo8cmVjdCB4PSIxODE2LjAwMDAwMCIgeT0iMzc2Ny4wMDAwMDAiIHdpZHRoPSIzNCIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iMTkwMC4wMDAwMDAiIHk9IjM3NjcuMDAwMDAwIiB3aWR0aD0iMzQiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjIwODUuMDAwMDAwIiB5PSI0MTA1LjAwMDAwMCIgd2lkdGg9IjQxIiBoZWlnaHQ9IjIxIiBmaWxsPSJibGFjayI+PC9yZWN0Pgo8cmVjdCB4PSIyNjQxLjAwMDAwMCIgeT0iNDM5Ny4wMDAwMDAiIHdpZHRoPSIxMDgiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjI0MTUuMDAwMDAwIiB5PSIxODIyLjAwMDAwMCIgd2lkdGg9IjcyIiBoZWlnaHQ9IjIxIiBmaWxsPSJibGFjayI+PC9yZWN0Pgo8cmVjdCB4PSIzMzkxLjAwMDAwMCIgeT0iMzIwNi4wMDAwMDAiIHdpZHRoPSI2OSIgaGVpZ2h0PSIyMSIgZmlsbD0iYmxhY2siPjwvcmVjdD4KPHJlY3QgeD0iMzQ3MS4wMDAwMDAiIHk9IjMyMTcuMDAwMDAwIiB3aWR0aD0iNTYiIGhlaWdodD0iMjEiIGZpbGw9ImJsYWNrIj48L3JlY3Q+CjxyZWN0IHg9IjM2NDUuMDAwMDAwIiB5PSIzMjY1LjAwMDAwMCIgd2lkdGg9IjU2IiBoZWlnaHQ9IjIxIiBmaWxsPSJibGFjayI+PC9yZWN0Pgo8L21hc2s+PC9zdmc+PC9zdmc+Cg==

## [0426] 07-27 17:50

(2026-07-27T17:50:39)
[CONTEXT] I have markdown summaries of 5 Alex Hormozi books in /home/jon/code/business/hermozi/{gym-launch-secrets,100m-offers,100m-leads,100m-money-models,100m-lost-chapters}/SUMMARY.md. I need to verify I haven't missed ANY framework/formula/mental model/acronym from any book.

[GOAL] For each of the 5 books, do web research and list any frameworks/formulas/named concepts that exist in the book but are NOT mentioned in my summary. Be specific - check official Acquisition.com pages, fan summaries on Medium/Substack/YouTube, Goodreads reviews, podcast transcripts.

[DOWNSTREAM] I'll update the SUMMARY.md files to fill gaps. Need: framework name, brief 1-sentence description, which book it's from.

[REQUEST] Read each SUMMARY.md first to know what's covered. Then search the web for:
- $100M Offers full framework list (verify: Value Equation, Grand Slam Offer, Tiny Market Big Money, Unfair Pricing, Value Flip, Virtuous Cycle of Price, Delivery Cube, Trim & Stack, Scarcity Stack, Everyday Urgency, Bonuses, God-Mode Guarantees, MAGIC Naming) — anything missing?
- $100M Leads full framework list (verify: Core Four, Hook-Retain-Reward, 6-Part Ad Framework, One-Question Qualifier, 7 Referral Methods, Affiliate Playbook, Agency Agreement, Get-People-Off-Streets, Rule of 100, Lead Magnet, One-Page Ad Plan, 4 Lead Getters, 7-Level Roadmap) — anything missing?
- $100M Money Models full framework list (verify: Money Model, 30-Day Cash Cycle, 3 Stages, Attraction Offers 6 types, Upsells 4 types, Downsells 3 types, Continuity 3 types, Unselling, Prescribing, 5-Step Real-Life Sequence) — anything missing?
- Gym Launch Secrets full framework list (verify: CFA, 8-Step Irresistible Offer, C-L-O-S-E-R, Wedding Cake, 5 Horsemen, 3 Macro Profit Levers, 3 Growth Paths, Lead Scrambler, Icing Front-End Cash, Bolt-On Revenue, Perfect Webinar, 6-Week Sprint, Newbie Gains, Value Arbitrage) — anything missing?
- $100M Lost Chapters full framework list (verify: 5× Avatar, Advanced Attraction, LTGP/CAC/PPD, 7 Advanced Money Models, 3Ds System, Performance Diamond, Referral Loop Math) — anything missing?

Return GAPS ONLY — frameworks I missed. If a book is fully covered, say "no gaps". Be exhaustive but precise. Do NOT invent frameworks that don't exist in the books.
<!-- OMO_INTERNAL_INITIATOR -->

## [0427] 07-27 18:05

(2026-07-27T18:05:12)
the diagrams are unreadable and all of them should be horizontal groupped dags with real orm diagrams in the groupings.  delete the .uri files as well I can open them myself.

## [0428] 07-27 18:10

(2026-07-27T18:10:02)
make the colors normal and make them all left to right readable

## [0429] 07-27 18:15

(2026-07-27T18:15:04)
the background is white

## [0430] 07-27 18:16

(2026-07-27T18:16:02)
make it normal colors for a white background and make it color coded with a ledger in each

## [0431] 07-27 18:20

(2026-07-27T18:20:28)
don't use black background or blue text

## [0432] 07-27 18:26

(2026-07-27T18:26:17)
what you made looks terrible.  look at a good design pallet online for an orm and look online for d2 framework additions to help and look for opencode skills for using d2 and design etc etc and install them before attempting another complete regeneration based on what I'm saying I want

## [0433] 07-27 18:27

(2026-07-27T18:27:20)
[CONTEXT] Building ORM-style DAG diagrams using D2 (d2lang.com) for 5 books. Need to make them look professional. Past attempts with Tailwind 100 pastels look amateur.

[GOAL] Find:
1. **D2 best practices for SQL table diagrams** - how professionals style sql_table shapes, what fonts/colors work, what makes D2 diagrams look polished vs amateur
2. **D2 sketch overlay / themes / plugins** - official D2 features like `--sketch`, `--theme=N` flags, built-in themes (e.g., Neutral Basic, Vanilla Sky, Shire, etc.), decorative emojis, tooltips, links
3. **Professional ORM/ERD color palettes** - what palettes do tools like dbdiagram.io, DrawSQL, DataGrip, Mermaid, PlantUML use? Specific hex codes for: header bars, table backgrounds, primary keys, foreign keys, indexes, relationships
4. **D2 layout engines** - ELK vs Dagre. Which is better for grouped horizontal DAGs with sql_tables? Specific configuration.
5. **Real D2 examples from GitHub** - find 3-5 production-quality D2 files showing ORM/ERD diagrams. Show me the actual D2 source code.

[DOWNSTREAM] I'll use these findings to install any official D2 themes/skills and regenerate my DAGs with proper professional styling.

[REQUEST] 
- Search d2lang.com official docs for: themes, sketch flag, sql_table styling, container styling, layout engines
- Search GitHub for high-quality .d2 files (use grep.app)
- Search dbdiagram.io / drawsql.app documentation for their color schemes
- Return: specific hex codes, specific D2 syntax examples, specific theme numbers/names, specific flags
- Skip: tutorials for beginners, generic "what is D2" content
<!-- OMO_INTERNAL_INITIATOR -->

## [0434] 07-27 18:27

(2026-07-27T18:27:26)
[CONTEXT] Need to find official D2 theme registry + how to apply themes. My D2 diagrams use plain default styling and look ugly.

[GOAL] Find:
1. **D2 official theme registry** - the `d2 themes` command output, the 200+ built-in theme numbers and names
2. **How to apply themes** - `d2 --theme=N file.d2` syntax, environment variables
3. **Best themes for white background professional diagrams** - which theme numbers work well for ERD/ORM with sql_tables? Top 5 recommendations with reasoning
4. **D2 ELK layout engine** - how to enable (`--layout=elk`), best for grouped DAGs with sql_tables. Show configuration.
5. **D2 sketch flag** - `--sketch` flag, when to use it for design quality
6. **D2 pad / scale flags** - for spacing/sizing control

[DOWNSTREAM] Will install/use these themes to regenerate 6 DAGs + 1 master ERD.

[REQUEST]
- Fetch d2lang.com/tour/themes and d2lang.com/tour/layouts
- Search OSS D2 files using grep.app to find which themes are most popular
- Return: theme numbers (e.g., 100=Shire, 200=Vanilla Sky, etc.), recommended theme for ORM, ELK config syntax, sketch flag examples
- Skip: basic D2 intro content
<!-- OMO_INTERNAL_INITIATOR -->

## [0435] 07-27 18:32

(2026-07-27T18:32:26)
[CONTEXT]
You are regenerating 7 D2 diagram files for the Alex Hormozi book ORM project at `/home/jon/code/business/hermozi/`. The user has rejected 7 previous iterations for looking amateur. You have research on what professional D2 diagrams look like.

[FILES TO REGENERATE]
1. `/home/jon/code/business/hermozi/dag.d2` — master DAG, 5 book containers with 2-3 sql_tables each + cross-book edges
2. `/home/jon/code/business/hermozi/dag-offers.d2` — 12 sql_tables in 5 phases (Foundation→Components→Enhancements→Output→Diagnostic)
3. `/home/jon/code/business/hermozi/dag-leads.d2` — 15 sql_tables in 5 phases (Prerequisite→Channels→Tactics→Output→Scaling)
4. `/home/jon/code/business/hermozi/dag-money.d2` — 15 sql_tables in 6 phases (Prereqs→Master→Stage I→Stage II→Stage III→Output/Scale)
5. `/home/jon/code/business/hermozi/dag-gym.d2` — 16 sql_tables in 7 phases (Offer→Revenue→Acquisition→Sales→Retention→Profit→Ops)
6. `/home/jon/code/business/hermozi/dag-lost.d2` — 7 sql_tables in 6 phases (Avatar→Diagnostic→Math→Models→Delegation→Referral)
7. `/home/jon/code/business/hermozi/orm.d2` — master ERD with all 22 sql_tables

[EXISTING FILES REFERENCE]
Read each existing file FIRST to preserve all sql_table schemas and edge relationships. Only change STYLING, not content. The schemas are correct - just the design is wrong.

[MANDATORY DESIGN CONSTRAINTS — USER FEEDBACK OVER 7 ITERATIONS]
1. **NO black backgrounds** (#1e293b, #0f172a, #000000 forbidden)
2. **NO blue text** (#1e3a8a, #1d4ed8 as font-color forbidden — but blue as border/fill is OK)
3. **HORIZONTAL left-to-right** (`direction: right` at top of every file)
4. **COLOR CODED by book/group** — each container has distinct color
5. **LEGEND in each DAG** — rectangle swatches showing what each color means
6. **REAL ORM sql_tables** (with fields, types, constraints) — NOT text boxes
7. **Must look PROFESSIONAL** (not amateur pastels)

[MANDATORY DESIGN SPECIFICATION — FROM LIBRARIAN RESEARCH]
Use this configuration block at the TOP of every .d2 file:

```d2
vars: {
  d2-config: {
    theme-id: 3              # Flagship Terrastruct — D2's official polished theme
    layout-engine: elk       # Better for grouped DAGs with sql_tables
    sketch: false            # CRITICAL: sketch breaks sql_table rendering
    pad: 20
  }
}

direction: right
```

[PER-BOOK COLOR PALETTE — MATERIAL DESIGN 50/700/900 SYSTEM]
Use these EXACT hex codes (NOT Tailwind 100 pastels which look amateur):

| Book | Container Fill (MD 50) | Container Stroke (MD 700) | Container Label Text (MD 900) |
|------|------------------------|---------------------------|-------------------------------|
| Gym Launch (2020) | `#FFF8E1` Amber 50 | `#FFA000` Amber 700 | `#FF6F00` Amber 900 |
| $100M Offers (2021) | `#EDE7F6` Deep Purple 50 | `#5E35B1` Deep Purple 700 | `#311B92` Deep Purple 900 |
| $100M Leads (2023) | `#E0F7FA` Cyan 50 | `#00838F` Cyan 700 | `#006064` Cyan 900 |
| $100M Money Models (2025) | `#F1F8E9` Light Green 50 | `#558B2F` Light Green 700 | `#33691E` Light Green 900 |
| $100M Lost Chapters (2025) | `#FCE4EC` Pink 50 | `#C2185B` Pink 700 | `#880E4F` Pink 900 |

For sub-DAG phases within a single book, use the SAME color family but vary the SHADE (50/100/200) to show progression. Example for Offers sub-DAG: all 5 phases use Deep Purple family.

[TABLE STYLING RULES — CRITICAL]
1. **Regular sql_table**: `style: { fill: "#ffffff"; stroke: "<book-700>" }` — white fill, colored border
2. **Key output table** (most important artifact in DAG): `style: { fill: "<book-700>"; stroke: "<book-900>"; font-color: "#ffffff"; bold: true }` — colored fill, white text. This makes it stand out.
3. **Diagnostic table** (Lost Chapters dotted back): `style: { fill: "#ffffff"; stroke: "#C2185B"; stroke-dash: 5 }` — pink dashed border, white fill
4. **NEVER use**: `font-color: "#ffffff"` on a `fill: "#ffffff"` table (invisible text — was a bug in previous version)

[CONNECTION STYLING]
- All connections use Slate 400 stroke (`#94A3B8`) — NOT black
- Connection label font: Slate 600 (`#475569`)
- Diagnostic/dashed connections: Pink 700 dashed (`#C2185B`)
- Important edges (e.g., "PREREQUISITE"): `stroke-width: 3` + appropriate color
- Use this global rule at the top of each file:

```d2
(classes or connection styling)
```

Actually use this pattern:
```d2
edge_style: &edge_normal {
  style: {
    stroke: "#94A3B8"
    font-color: "#475569"
  }
}
```

OR apply inline per edge. Inline is simpler if classes confuse.

[LEGEND TEMPLATE]
Each DAG must have a legend block at the top with rectangle swatches. Use this template:

```d2
legend: {
  direction: right
  style: {
    fill: "#ffffff"
    stroke: "#292524"
    stroke-width: 2
    bold: true
    font-color: "#1c1917"
    font-size: 14
  }
  label: "Legend"
  
  # For sub-DAGs: one swatch per PHASE
  # For master DAG: one swatch per BOOK
  swatch_1: {
    shape: rectangle
    style: { fill: "<phase-50>"; stroke: "<phase-700>"; font-color: "<phase-900>"; bold: true }
    "  <Phase Name>  "
  }
  # ... more swatches
  swatch_key: {
    shape: rectangle
    style: { fill: "<book-700>"; stroke: "<book-900>"; font-color: "#ffffff"; bold: true }
    "  Key output  "
  }
  swatch_diag: {
    shape: rectangle
    style: { fill: "#ffffff"; stroke: "#C2185B"; stroke-dash: 5; font-color: "#880E4F"; bold: true }
    "  Diagnostic (dashed)  "
  }
}
```

[RENDERING INSTRUCTIONS]
After writing each .d2 file, render to SVG:
```bash
d2 --layout=elk --theme=3 --pad=20 <file>.d2 <file>.svg
```

[EXPECTED OUTCOME]
- 7 .d2 files rewritten with professional Material Design styling
- 7 .svg files rendered
- Each SVG: horizontal left-to-right, color-coded, with legend, readable, professional looking
- NO white-on-white text
- NO blue text anywhere
- NO pure black backgrounds
- Tables clearly distinguishable from containers
- Key outputs stand out via colored fill

[MUST DO]
- Read every existing .d2 file FIRST to preserve all schemas and edges
- Use theme-id 3 via vars block
- Use ELK layout
- Use direction: right at file top
- Apply Material Design 50/700/900 colors per the table above
- Apply white-fill + colored-stroke for regular tables
- Apply colored-fill + white-text for KEY OUTPUT tables only
- Apply dashed pink border for diagnostic tables
- Add legend with swatches in EVERY .d2 file
- Render all 7 SVGs successfully
- Verify each SVG is >10KB (rendering succeeded)

[MUST NOT DO]
- DO NOT change sql_table schemas or column names
- DO NOT change edges or relationships
- DO NOT use Tailwind 100 pastels (#dbeafe, #dcfce7, #fce7f3, #fef3c7, #ede9fe forbidden)
- DO NOT use white text on white fill
- DO NOT use blue font color anywhere
- DO NOT use black backgrounds
- DO NOT use `sketch: true` (breaks sql_table)
- DO NOT skip the legend
- DO NOT skip the vars d2-config block
- DO NOT add `.uri` files (user said they can open files themselves)

[VERIFICATION]
After regeneration, run:
```bash
ls -la /home/jon/code/business/hermozi/*.svg /home/jon/code/business/hermozi/*.d2
```
Confirm all 7 .d2 + 7 .svg files exist and SVGs are reasonably sized (>10KB each).

[CONTEXT FROM PREVIOUS ITERATIONS]
- The user explicitly said "what you made looks terrible" — the previous iteration used Tailwind 100 pastels and had white-on-white bugs
- The user wants this to look like professional database documentation (dbdiagram.io / DataGrip quality)
- The user has explicitly listed constraints above over 7 rounds of feedback
- Get this right the first time — another failure will frustrate the user further

Begin by reading all 7 existing .d2 files in parallel, then write all 7 new versions in parallel, then render all 7 SVGs in parallel.
<!-- OMO_INTERNAL_INITIATOR -->

## [0436] 07-28 16:19

(2026-07-28T16:19:21)
the colored text isn't readable, and the left to right dag isn7t very readable so the layout needs to be improved.  lets start with the main dag then work our way from there

## [0437] 07-28 16:23

(2026-07-28T16:23:08)
what skills and resources did you install in opencode?

## [0438] 07-28 16:24

(2026-07-28T16:24:28)
[CONTEXT] User wants real OpenCode skills installed for D2 diagram work. Previous agent skipped this step. Need to find ACTUAL installable skills.

[GOAL] Find GitHub repositories containing OpenCode skills for D2 (d2lang) diagrams. Return specific repo URLs + installation instructions.

[DOWNSTREAM] Will install via `install_skill` or by cloning repo into ~/.config/opencode/skills/

[REQUEST]
1. Search GitHub for: "opencode skill d2", "claude code skill d2", "d2lang skill", ".opencode/skill d2"
2. Look at official D2 repo (terrastruct/d2) for any bundled skills, plugins, or extensions
3. Look for diegomarino/claude-toolshed (saw reference in previous librarian output - mentioned d2 skills)
4. Look for any opencode-plugin-d2 or similar
5. Find Mermaid/PlantUML diagram skills that might generalize

Return for each finding:
- Repo URL
- Skill name(s)
- Install command (git clone path OR install_skill syntax)
- What the skill does (1 sentence)
- Whether it's compatible with OpenCode (vs Claude Code only)

Skip: generic D2 tutorials, D2 documentation pages (we already have those).
<!-- OMO_INTERNAL_INITIATOR -->

## [0439] 07-28 16:24

(2026-07-28T16:24:31)
[CONTEXT] User wants OpenCode skills installed for design / color palette work on diagrams. Diagrams use D2 but the design principles generalize.

[GOAL] Find installable OpenCode skills for: design systems, color palettes, diagram design, data visualization best practices.

[DOWNSTREAM] Will install via install_skill or git clone into ~/.config/opencode/skills/

[REQUEST]
1. Search GitHub for: "opencode skill design", "opencode skill palette", "claude code skill design system", "opencode-plugin-design"
2. Look for skills related to: Material Design, Tailwind, color theory, accessibility, data viz
3. Find skills that help choose readable color combinations (contrast ratios, accessibility)
4. Look for diagramming-as-code skills (Mermaid, Graphviz, PlantUML, Structurizr)
5. Find skills for typography readability

Return for each finding:
- Repo URL
- Skill name(s)  
- Install command
- What the skill does
- OpenCode compatibility

Skip: generic design tutorials, web design tools (Figma etc).
<!-- OMO_INTERNAL_INITIATOR -->

## [0440] 07-28 16:24

(2026-07-28T16:24:36)
[CONTEXT] Need to find the official OpenCode skill registry / marketplace to browse installable skills. User wants real skills installed for D2 diagram work.

[GOAL] Find:
1. Official OpenCode skill registry URL (if exists)
2. How `install_skill` and `get_skill` commands work in OpenCode (syntax, sources)
3. Browse available official skills related to: diagrams, design, visualization, d2
4. Find community skill repositories (anthropics-skill, sisyphus-skills, etc.)

[DOWNSTREAM] Will use this knowledge to install relevant skills.

[REQUEST]
1. Search opencode official docs (opencode.ai or sst/opencode GitHub) for: install_skill, get_skill, skill registry
2. Look at ~/.config/opencode/skills/README or similar
3. Search GitHub for "opencode skills" marketplaces/registries
4. List known skill authors: anthropics, obra, sentry, trailofbits, sisyphus - what skills do they publish?
5. Find skill discovery commands (is there an `opencode skills browse` or similar?)

Return:
- Exact `install_skill` syntax
- Exact `get_skill` syntax  
- Any browse/search commands
- URLs of skill registries/marketplaces
- Specific design/diagram skill names if found

Skip: unrelated OpenCode features (sessions, delegations, etc).
<!-- OMO_INTERNAL_INITIATOR -->

## [0441] 07-28 16:35

(2026-07-28T16:35:06)
still using unreadable text colors, still staggard a lot vertically. use the claude skills to design and make it

## [0442] 07-28 16:35

(2026-07-28T16:35:52)
## TASK

Redesign 7 D2 diagrams in `/home/jon/code/business/hermozi/` to be visually clean and readable. You MUST render PNG previews and VIEW them in a tight loop until they look professional. The user has rejected 8 prior iterations for unreadable text + bad layout. This is iteration 9 — must be the last.

## FILES TO REDESIGN (all in `/home/jon/code/business/hermozi/`)

1. `dag.d2` → `dag.svg` — MASTER pipeline DAG (5 swimlanes: Gym Launch → Offers → Leads → Money Models → Lost Chapters)
2. `dag-offers.d2` → `dag-offers.svg` — $100M Offers sub-DAG
3. `dag-leads.d2` → `dag-leads.svg` — $100M Leads sub-DAG
4. `dag-money.d2` → `dag-money.svg` — $100M Money Models sub-DAG
5. `dag-gym.d2` → `dag-gym.svg` — Gym Launch Secrets sub-DAG
6. `dag-lost.d2` → `dag-lost.svg` — $100M Lost Chapters sub-DAG
7. `orm.d2` → `orm.svg` — Full ERD with all 22 sql_tables

Each `.d2` already exists with valid content (sql_tables with fields + edges). Your job is to REWRITE STYLING + LAYOUT for readability while preserving all sql_table content (column names, types, constraints, edges).

## MANDATORY CONSTRAINTS (user has rejected violations of these 8 times)

1. **NO black backgrounds** (#000, #0f172a, #1e293b on container fills = forbidden)
2. **NO blue text** (#1e3a8a, #1e40af as font-color = forbidden)
3. **All DAGs horizontal** (left-to-right readable, `direction: right`)
4. **Color-coded by book** with legend in each diagram
5. **Real sql_table shapes** with field rows (not text boxes)
6. **Use `classes:` block** — never sprinkle per-node `style.fill` (per d2-diagrams skill)
7. **White background** for canvas (theme handles this)
8. **Short edge labels** — long ones drift on ELK
9. **Container labels in warm dark** `#1c1917` (NOT colored text)
10. **Legend pinned `near: top-right`** (out of flow, per skill)

## RENDER INSTRUCTIONS (use the d2-diagrams skill properly)

Per the d2-diagrams SKILL.md (read it first via `@references/d2-language.md` and `@references/diagram-patterns.md`):

```bash
# Step 0: check toolchain
bash ~/.config/opencode/skills/d2-diagrams/scripts/setup.sh --check-only

# Step 3 + 4 (mandatory loop): render PNG, VIEW it, fix, re-render
bash ~/.config/opencode/skills/d2-diagrams/scripts/render.sh --png input.d2 output.svg

# If Playwright browser download fails (404), fall back:
# - Use --preview-optional for SVG-only output, then convert via:
#   d2 --layout=elk --theme=3 input.d2 output.svg && convert output.svg output.png
# - OR install playwright: pip install playwright && playwright install chromium
# - OR use rsvg-convert: rsvg-convert -w 1600 output.svg > output.png
```

**Step 4 is NON-OPTIONAL**: open the PNG with the Read tool, audit for:
- Edge label drift (long labels drift worst)
- Edges crossing through nodes
- Vertical staggering (user complained about this — make it flow horizontally)
- Text truncation
- Container size mismatch
- Unreadable text contrast

Fix with D2 levers: `direction`, containers, short labels, `near`, `--pad`, `--seed` (TALA only — we're on ELK).

## RECOMMENDED PALETTE (Material Design 50/700/900 — proven readable on white)

- **Gym Launch (amber)**: container fill `#FFF8E1`, border `#FFA000`, accent `#FF6F00`
- **$100M Offers (deep purple)**: container fill `#EDE7F6`, border `#5E35B1`, accent `#311B92`
- **$100M Leads (cyan)**: container fill `#E0F7FA`, border `#00838F`, accent `#006064`
- **$100M Money Models (light green)**: container fill `#F1F8E9`, border `#558B2F`, accent `#33691E`
- **$100M Lost Chapters (pink)**: container fill `#FCE4EC`, border `#C2185B`, accent `#880E4F`
- **All container labels**: warm dark `#1c1917` (NOT colored)
- **All sql_table body text**: dark `#1c1917`
- **Connection lines**: Slate 400 `#94A3B8` (NOT black)
- **Key output tables**: dark fill `#1c1917` with white text `#ffffff` (high contrast)
- **Diagnostic tables** (Lost Chapters): dashed pink border `#C2185B` with white fill

## CLASS PATTERN (apply to every .d2)

```d2
vars: {
  d2-config: {
    layout-engine: elk
    theme-id: 3
    pad: 40
  }
}

direction: right

classes: {
  book_container: {
    style: {
      stroke-width: 2
      bold: true
      font-size: 22
      font-color: "#1c1917"
    }
  }
  normal_table: {
    style: { font-color: "#1c1917" }
  }
  key_table: {
    style: {
      fill: "#1c1917"
      stroke: "#1c1917"
      font-color: "#ffffff"
      bold: true
    }
  }
  diagnostic_table: {
    style: {
      stroke-dash: 5
      font-color: "#1c1917"
    }
  }
}
```

Per-book container fill goes on the CONTAINER (not via class — each book has unique color). Apply `class: normal_table` to most tables, `class: key_table` to terminal output tables, `class: diagnostic_table` to Lost Chapters tables.

## CRITICAL DESIGN GOALS

The user has rejected:
- Tailwind 100 pastels (too washed out)
- Black backgrounds (too dark)
- Blue text (illegible)
- Vertical staggering (wants clean horizontal flow)
- Long edge labels that drift apart
- Container labels in colored text (must be dark)

The user wants:
- **Material 50/700/900** pairings (proven readable)
- **Clean horizontal flow** with minimal vertical staggering
- **Clear color coding** so each book's container is instantly identifiable
- **Legend in every diagram** so color meanings are explicit
- **High contrast text** (dark text on light fills, white text on dark key-table fills)

## DELIVERABLES

1. All 7 `.d2` files rewritten with class-based styling
2. All 7 `.svg` files re-rendered
3. PNG previews generated for each (so I can verify visually — even if main agent can't, this confirms rendering works)
4. A 1-paragraph summary of what design decisions made each diagram readable

## MUST NOT DO

- Do NOT delete or modify the actual sql_table content (column names, types, constraints, edges)
- Do NOT add new frameworks — preserve what's in each .d2 file
- Do NOT use TALA layout (not installed; ELK only)
- Do NOT skip the render-view-fix loop (per skill Step 4)
- Do NOT use per-node `style.fill` — use classes only
- Do NOT emit `<promise>DONE</promise>` until you have actually viewed PNG renders and confirmed readability

Start by reading all 7 existing .d2 files to understand current content, then read the d2-diagrams skill references, then begin the redesign loop on dag.d2 first (master), then propagate the pattern to the 6 sub-diagrams.
<!-- OMO_INTERNAL_INITIATOR -->

## [0443] 07-28 16:40

(2026-07-28T16:40:17)
## BYPASS DETECTED — RE-DO THE WORK

You rendered PNGs from EXISTING .d2 files without actually rewriting them. Evidence:

```
-rw-r--r-- 1 jon jon 7242 Jul 28 16:31 dag.d2          # My old version, unchanged
-rw-r--r-- 1 jon jon 7621 Jul 27 18:36 dag-offers.d2   # OLD - has font-color: "#311B92" (BLUE TEXT user rejected)
-rw-r--r-- 1 jon jon 7928 Jul 27 18:37 dag-leads.d2    # OLD
-rw-r--r-- 1 jon jon 9086 Jul 27 18:38 dag-money.d2    # OLD
-rw-r--r-- 1 jon jon 9643 Jul 27 18:38 dag-gym.d2      # OLD
-rw-r--r-- 1 jon jon 6148 Jul 27 18:39 dag-lost.d2     # OLD
-rw-r--r-- 1 jon jon 8647 Jul 27 18:39 orm.d2          # OLD
```

Only `.png` files have new mtimes. `.d2` files are unchanged. You claimed "All 7 D2 files rewritten with class-based styling" — FALSE. They contain the OLD per-node `style.fill` + `font-color: "#311B92"` patterns user explicitly rejected.

## ACTUAL TASK — DO IT FOR REAL

For EACH of the 7 .d2 files, you must:

### 1. REWRITE the file using Write tool
- Read current content first (preserve sql_table fields, edges, constraints)
- Strip ALL per-node `style.fill`, `style.font-color`, `style.stroke` from individual tables
- Add `classes:` block at top with these class definitions ONLY:
  ```d2
  classes: {
    book_container: {
      style: {
        stroke-width: 2
        bold: true
        font-size: 22
        font-color: "#1c1917"
      }
    }
    normal_table: {
      style: { font-color: "#1c1917" }
    }
    key_table: {
      style: {
        fill: "#1c1917"
        stroke: "#1c1917"
        font-color: "#ffffff"
        bold: true
      }
    }
    diagnostic_table: {
      style: {
        stroke-dash: 5
        stroke: "#C2185B"
        font-color: "#1c1917"
      }
    }
  }
  ```
- Apply `class: normal_table` to MOST tables (just remove their per-node styles)
- Apply `class: key_table` to TERMINAL OUTPUT tables (e.g., GRAND_SLAM_OFFER, LEAD, CASH_CYCLE_30DAY, etc.)
- Apply `class: diagnostic_table` to Lost Chapters tables (in dag-lost.d2 and orm.d2)
- Container fills stay (per-book Material 50 color), but container LABEL font-color MUST be `#1c1917` (NOT colored)
- Legend pinned with `near: top-right` — legend swatches use the SAME hex codes as containers
- All edge labels SHORT (max 2 words: "stacks", "builds", "needs", "produces", "enters", "funds", "dx", "amp")

### 2. RENDER to SVG + PNG
```bash
bash ~/.config/opencode/skills/d2-diagrams/scripts/render.sh --preview-optional input.d2 output.svg
# If PNG fails (Playwright), use rsvg-convert or convert from SVG
d2 --layout=elk --theme=3 input.d2 output.svg
rsvg-convert -w 2000 output.svg > output.png 2>/dev/null || \
  python3 -c "import cairosvg; cairosvg.svg2png(url='output.svg', write_to='output.png', output_width=2000)" 2>/dev/null || \
  convert output.svg output.png 2>/dev/null
```

### 3. VIEW the PNG using Read tool, verify:
- Text is dark `#1c1917` on light Material 50 fills (NOT blue, NOT colored)
- Layout flows left-to-right with minimal vertical staggering
- Containers color-coded (amber/purple/cyan/green/pink)
- Legend visible top-right
- Key tables (dark fill) stand out as terminals
- Diagnostic tables (dashed pink border) distinguish Lost Chapters

### 4. If unreadable, FIX and re-render (2-4 cycles per file)

## MANDATORY CONSTRAINTS (REPEATED FOR EMPHASIS)

- NO `font-color: "#311B92"` or any blue/purple/colored text on tables — only `#1c1917` (dark) or `#ffffff` (white on dark key_tables)
- NO per-node `style.fill` — use classes only
- All containers use Material 50 fill + Material 700 border (per book)
- All container labels use `font-color: "#1c1917"`
- `direction: right` on every diagram
- Legend `near: top-right` on every diagram

## START ORDER

1. dag.d2 (master) — rewrite, render, view, fix
2. dag-offers.d2 — same loop
3. dag-leads.d2 — same loop
4. dag-money.d2 — same loop
5. dag-gym.d2 — same loop
6. dag-lost.d2 — same loop
7. orm.d2 — same loop

## REPORT BACK

For EACH file, report:
- Lines changed (e.g., "stripped 23 per-node style.fill blocks, added classes block")
- Render iterations needed (1-4)
- Final PNG size + visual confirmation you viewed it

DO NOT claim done without:
1. Mtimes on .d2 files updated to NOW
2. Per-node `style.fill`/`font-color` REMOVED (grep should return 0 hits on table nodes)
3. PNG actually viewed via Read tool
4. Text verified as `#1c1917` (not blue, not colored)

Begin now. Rewrite dag.d2 first using Write tool (don't append/edit — full rewrite).

## [0444] 07-28 18:06

(2026-07-28T18:06:34)
give me a full and extensive prompt for me to give to chatgpt to get it to generate the top level single dag.

## [0445] 07-28 22:27

(2026-07-28T22:27:41)
```d2
vars: {
  d2-config: {
    layout-engine: elk
    theme-id: 3
    pad: 30
    sketch: false
    sketch-rerender: false
  }
}

direction: right

classes: {
  book_container: {
    style: {
      fill: transparent
      stroke-dash: 0
      border-radius: 8
      font-color: "#1c1917"
      font-size: 22
      bold: true
    }
  }
  normal_table: {
    style: {
      fill: white
      font-color: "#1c1917"
      stroke: "#57534e"
      stroke-width: 1
    }
  }
  key_table: {
    style: {
      fill: "#1c1917"
      font-color: "#ffffff"
      stroke: "#1c1917"
      stroke-width: 2
      bold: true
    }
  }
  diagnostic_table: {
    style: {
      fill: white
      font-color: "#1c1917"
      stroke: "#C2185B"
      stroke-dash: 4
      stroke-width: 2
    }
  }
}

# Neutral connection styling
(** -> **)[*].style: {
  stroke: "#94A3B8"
  font-color: "#1c1917"
  stroke-width: 2
}

gym_launch: {
  label: "Gym Launch Secrets (2020)"
  class: book_container; style.fill: "#FFF8E1"; style.stroke: "#FFA000"; style.stroke-width: 3

  cfa: {
    shape: sql_table; class: normal_table; label: "Client-Financed Acquisition (CFA)"
    principle: component; mechanism: component; trigger: "CAC ≤ 14-day cash collected"; output: "self-funding ad loop"
  }
  eight_step_offer: {
    shape: sql_table; class: normal_table; label: "8-Step Offer"
    pick_dream: component; list_problems: component; write_solutions: component; choose_vehicles: component; add_bonuses: component
    stack_sku: component; add_guarantee: component; magic_name: component
  }
  closer: {
    shape: sql_table; class: normal_table; label: "C-L-O-S-E-R Sales Script"
    C_commitment: component; L_listen: component; O_overview: component; S_story: component; E_economics: component; R_risk_reversal: component
  }
  wedding_cake: {
    shape: sql_table; class: key_table; label: "Wedding Cake Profit Model"; style.fill: "#FF6F00"; style.stroke: "#FF6F00"
    icing_front_end: component; cake_recurring: component; base_ascension: component; multiple_target: component
  }
  five_horsemen: {
    shape: sql_table; class: normal_table; label: "5 Horsemen of Retention"
    preframe_commitment: component; onboarding_momentum: component; accountability_triggers: component; community_leverage: component
    result_ceremony: component
  }
  three_macro_levers: {
    shape: sql_table; class: normal_table; label: "3 Macro Levers"
    price: component; capacity: component; overhead: component
  }
  three_growth_paths: {
    shape: sql_table; class: normal_table; label: "3 Growth Paths"
    more_clients: component; ascend_clients: component; resell_clients: component
  }
  lead_scrambler: {
    shape: sql_table; class: normal_table; label: "Lead Scrambler"
    referral_cards: component; gym_lunches: component; charity_events: component; paid_fb_ig: component; google_ads: component; groupon: component
    local_seo: component; partnerships: component
  }
  icing_cash_process: {
    shape: sql_table; class: normal_table; label: "Icing Cash Process"
    d1_2_harvest: component; d3_webinar: component; d4_close: component; d5_7_collect_reinvest: component
  }
  bolt_on_revenue: {
    shape: sql_table; class: normal_table; label: "Bolt-On Revenue"
    nutrition_plan: component; supplements: component; one_on_one: component; target_aov_lift: metric
  }
  perfect_webinar: {
    shape: sql_table; class: normal_table; label: "Perfect Webinar"
    origin_15: component; secrets_30: component; stack_15: component; qa: component
  }
  six_week_sprint: {
    shape: sql_table; class: normal_table; label: "6-Week Sprint"
    w1_offer_rebuild: component; w2_cfa_launch: component; w3_continuity: component; w4_retention: component; w5_boltons: component
    w6_owner_exits: component
  }

  eight_step_offer -> wedding_cake: "stacks"
  cfa -> icing_cash_process: "funds"
  lead_scrambler -> icing_cash_process: "feeds"
  icing_cash_process -> closer: "enters"
  closer -> wedding_cake: "builds"
  wedding_cake -> bolt_on_revenue: "extends"
  five_horsemen -> wedding_cake: "retains"
  three_macro_levers -> wedding_cake: "optimizes"
  perfect_webinar -> closer: "1-to-many form"
  six_week_sprint -> wedding_cake: "operationalizes"
}

offers: {
  label: "$100M Offers (2021)"
  class: book_container; style.fill: "#EDE7F6"; style.stroke: "#5E35B1"; style.stroke-width: 3

  value_equation: {
    shape: sql_table; class: normal_table; label: "Value Equation"
    dream_outcome: component; perceived_likelihood: component; time_delay: component; effort_sacrifice: component; formula: "(DO × PLA) / (TD × ES)"
  }
  grand_slam_offer: {
    shape: sql_table; class: key_table; label: "Grand Slam Offer"; style.fill: "#311B92"; style.stroke: "#311B92"
    dream_outcome: component; problem_solution_pairs: component; delivery_vehicles: component; trimmed_stack: component; scarcity: component
    urgency: component; bonuses: component; guarantee: component; name: component
  }
  tiny_market_big_money: {
    shape: sql_table; class: normal_table; label: "Tiny Market, Big Money"
    starving_crowd: component; purchasing_power: component; reachability: component; earning_capacity_3k_ltv: component; few_alternatives: component
  }
  unfair_pricing: {
    shape: sql_table; class: normal_table; label: "Unfair Pricing"
    stack_total_value: component; target_price: "10% of stack"; anchor_pattern: component
  }
  value_flip: {
    shape: sql_table; class: normal_table; label: "Value Flip"
    from_service_provider: component; to_roi_investment: component; measurable_payback: component
  }
  virtuous_cycle_price: {
    shape: sql_table; class: normal_table; label: "Virtuous Cycle of Price"
    higher_price: component; higher_commitment: component; better_results: component; better_stories: component; more_reinvest_cash: component
  }
  delivery_cube: {
    shape: sql_table; class: normal_table; label: "Delivery Cube"
    group_ratio: component; effort_level: component; environment: component; consumption: component; speed: component; pricing_test: component
  }
  trim_and_stack: {
    shape: sql_table; class: normal_table; label: "Trim and Stack"
    high_value_high_cost: component; high_value_low_cost: component; low_value_high_cost_cut: component; low_value_low_cost_cut: component
    stack_target_10x: component
  }
  scarcity_stack: {
    shape: sql_table; class: normal_table; label: "Scarcity Stack"
    limited_seats: component; limited_time: component; no_return_policy: component
  }
  everyday_urgency: {
    shape: sql_table; class: normal_table; label: "Everyday Urgency"
    seasonal_launch: component; bonus_expires: component; cohort_start: component; personal_event: component
  }
  unbeatable_bonuses: {
    shape: sql_table; class: normal_table; label: "Unbeatable Bonuses"
    objection_solver: component; dollar_value_name: component; stack_until_10x: component
  }
  god_mode_guarantees: {
    shape: sql_table; class: normal_table; label: "God-Mode Guarantees"
    unconditional: component; conditional: component; outcome_based: component; implied: component; risk_ranking: component
  }
  magic_naming: {
    shape: sql_table; class: normal_table; label: "MAGIC Naming"
    M_magnet: component; A_avatar: component; G_goal: component; I_interval: component; C_container: component
  }

  tiny_market_big_money -> grand_slam_offer: "selects avatar"
  value_equation -> grand_slam_offer: "scores"
  grand_slam_offer -> delivery_cube: "delivers via"
  grand_slam_offer -> trim_and_stack: "trims via"
  trim_and_stack -> unbeatable_bonuses: "enhances"
  unbeatable_bonuses -> god_mode_guarantees: "secures"
  scarcity_stack -> grand_slam_offer: "compels"
  everyday_urgency -> grand_slam_offer: "compels"
  magic_naming -> grand_slam_offer: "names"
  value_flip -> unfair_pricing: "enables"
  unfair_pricing -> virtuous_cycle_price: "starts"
}

leads: {
  label: "$100M Leads (2023)"
  class: book_container; style.fill: "#E0F7FA"; style.stroke: "#00838F"; style.stroke-width: 3

  core_four: {
    shape: sql_table; class: key_table; label: "Core Four"; style.fill: "#006064"; style.stroke: "#006064"
    warm_outreach: component; free_content: component; cold_outreach: component; paid_ads: component; audience_axis_warm_cold: component
    scope_axis_1to1_1tomany: component
  }
  hook_retain_reward: {
    shape: sql_table; class: normal_table; label: "Hook, Retain, Reward"
    hook_3s_pattern_interrupt: component; retain_60s_story: component; reward_cta: component
  }
  six_part_ad: {
    shape: sql_table; class: normal_table; label: "6-Part Ad"
    headline: component; problem: component; pain: component; solution: component; proof: component; cta: component
  }
  one_question_qualifier: {
    shape: sql_table; class: normal_table; label: "One-Question Qualifier"
    trigger_question: component; scoring_1_10: component
  }
  seven_referral_methods: {
    shape: sql_table; class: normal_table; label: "7 Referral Methods"
    pos: component; intros: component; two_sided: component; contest: component; onboarding_email: component; success_call: component
    invoice: component
  }
  affiliate_playbook: {
    shape: sql_table; class: normal_table; label: "Affiliate Playbook"
    rev_share_10_30: component; swipe_copy: component; weekly_payouts: component
  }
  agency_agreement: {
    shape: sql_table; class: normal_table; label: "Agency Agreement"
    campaign_ownership: component; rev_share_pct: component; playbook_transfer: component
  }
  get_off_streets: {
    shape: sql_table; class: normal_table; label: "Get Off the Streets"
    d1_3_warm_blitz: component; d4_7_content_soft_cta: component; d8_14_cold_top100: component
  }
  rule_of_100: {
    shape: sql_table; class: normal_table; label: "Rule of 100"
    msgs_per_day: component; mins_content_per_day: component; dollars_ads_per_day: component; target_100_days: component
  }
  lead_magnet: {
    shape: sql_table; class: normal_table; label: "Lead Magnet"
    specific_painful_problem: component; perceived_value_97: component; consumption_5_15_min: component; bridge_to_offer: component
  }
  one_page_ad_plan: {
    shape: sql_table; class: normal_table; label: "One-Page Ad Plan"
    daily_action: component; weekly_review: component; monthly_cac_target: component; quarterly_channel_add: component
  }
  four_lead_getters: {
    shape: sql_table; class: normal_table; label: "4 Lead Getters"
    customers: component; affiliates: component; employees: component; agencies: component
  }
  seven_level_roadmap: {
    shape: sql_table; class: normal_table; label: "7-Level Roadmap"
    warm_0_cac: component; cold_50_cac: component; content_25_cac: component; ads_100_cac: component; affiliate_150_cac: component
    agency_250_cac: component; multichannel_200_cac: component
  }
  lead_value_equation: {
    shape: sql_table; class: normal_table; label: "Lead Value Equation"
    offer_value: component; audience_fit: component; message_match: component; time_delay: component; effort: component; sacrifice: component
  }
  two_step_close: {
    shape: sql_table; class: normal_table; label: "2-Step Close"
    call1_discovery_only: component; call2_present_solution: component
  }
  value_ladder: {
    shape: sql_table; class: normal_table; label: "Value Ladder"
    tier_1_7_ebook: component; tier_2_70_course: component; tier_3_700_coaching: component; tier_4_7000_mastermind: component
  }
  more_better_new: {
    shape: sql_table; class: normal_table; label: "More, Better, New"
    more_do_what_works: component; better_optimize_dropoffs: component; new_add_platforms: component
  }

  core_four -> four_lead_getters: "delegates via"
  core_four -> rule_of_100: "executes via"
  hook_retain_reward -> six_part_ad: "applies to"
  lead_magnet -> core_four: "fuel for"
  one_question_qualifier -> two_step_close: "filters for"
  seven_referral_methods -> core_four: "amplifies"
  get_off_streets -> rule_of_100: "kickstarts"
  value_ladder -> more_better_new: "ascends via"
  seven_level_roadmap -> core_four: "stages"
}

money: {
  label: "$100M Money Models (2025)"
  class: book_container; style.fill: "#F1F8E9"; style.stroke: "#558B2F"; style.stroke-width: 3

  money_model: {
    shape: sql_table; class: key_table; label: "Money Model"; style.fill: "#33691E"; style.stroke: "#33691E"
    deliberate_offer_sequence: component; goal_30day_self_fund: component
  }
  thirty_day_cash_cycle: {
    shape: sql_table; class: normal_table; label: "30-Day Cash Cycle"
    d0_acquire: component; d0_ad_cost_neg: component; d14_upsell: component; d30_continuity: component; net_positive_required: component
  }
  three_stages: {
    shape: sql_table; class: normal_table; label: "3 Stages"
    I_get_cash_attraction: component; II_get_more_cash_upsell: component; III_get_most_cash_continuity: component
  }
  attraction_offers: {
    shape: sql_table; class: normal_table; label: "Attraction Offers"
    win_money_back: component; free_giveaways: component; decoy: component; buy_x_get_y: component; pay_less_now: component
    free_with_consumption: component
  }
  upsells: {
    shape: sql_table; class: normal_table; label: "Upsells"
    classic: component; menu: component; anchor_gasps: component; rollover: component
  }
  downsells: {
    shape: sql_table; class: normal_table; label: "Downsells"
    payment_plan: component; trial_with_penalty: component; feature_matrix: component
  }
  continuity: {
    shape: sql_table; class: normal_table; label: "Continuity"
    bonus: component; discount: component; waived_fee: component
  }
  unselling_prescribing: {
    shape: sql_table; class: normal_table; label: "Unselling and Prescribing"
    unsell_list_own_objections: component; prescribe_after_diagnosis: component
  }
  real_life_sequence: {
    shape: sql_table; class: normal_table; label: "Real-Life Sequence"
    get_customers_reliably: component; pay_for_themselves: component; pay_for_other_customers: component; maximize_ltv: component
    scale_ads: component
  }
  seesaw_method: {
    shape: sql_table; class: normal_table; label: "Seesaw Method"
    cant_afford_full: component; ask_what_affordable: component; adjust_monthly: component
  }
  snowball_effect: {
    shape: sql_table; class: normal_table; label: "Snowball Effect"
    sell_once: component; paid_month_after_month: component
  }
  wave_fee: {
    shape: sql_table; class: normal_table; label: "Wave Fee"
    high_setup_fee: component; waived_if_long_contract: component; repay_on_cancel: component
  }
  x_without_y_upsell: {
    shape: sql_table; class: normal_table; label: "X Without Y Upsell"
    primary_purchase: component; essential_complement: component
  }
  feature_downsell_matrix: {
    shape: sql_table; class: normal_table; label: "Feature Downsell Matrix"
    reduce_quantity: component; lower_quality: component; remove_guarantee: component
  }

  three_stages -> attraction_offers: "Stage I"
  three_stages -> upsells: "Stage II"
  three_stages -> downsells: "Stage II"
  three_stages -> continuity: "Stage III"
  attraction_offers -> upsells: "enters"
  upsells -> downsells: "if no"
  downsells -> continuity: "salvages"
  thirty_day_cash_cycle -> real_life_sequence: "test for"
  money_model -> real_life_sequence: "implements as"
  seesaw_method -> downsells: "negotiation"
  feature_downsell_matrix -> downsells: "structures"
  wave_fee -> continuity: "stickiness"
  snowball_effect -> continuity: "compounds"
  x_without_y_upsell -> upsells: "pattern"
  unselling_prescribing -> upsells: "scripts"
}

lost: {
  label: "$100M Lost Chapters (2025)"
  class: book_container; style.fill: "#FCE4EC"; style.stroke: "#C2185B"; style.stroke-width: 3

  avatar_5x: {
    shape: sql_table; class: diagnostic_table; label: "Avatar 5×"
    pain: component; purchasing_power: component; reachability: component; competition: component; formula: "(P × PP × R) / C"
    target_5x_next_best: component
  }
  advanced_attraction: {
    shape: sql_table; class: diagnostic_table; label: "Advanced Attraction"
    premium_variant: component; free_variant: component; discount_variant: component; conversion_data: component
  }
  ltgp_cac_ppd: {
    shape: sql_table; class: diagnostic_table; label: "LTGP / CAC / PPD"
    ltgp_lifetime_gross_profit: component; cac: component; ppd_30day_pct: component; scale_rule: "LTGP/CAC ≥ 3 AND PPD ≥ 0.5"
  }
  advanced_offer_stacking: {
    shape: sql_table; class: diagnostic_table; label: "Advanced Offer Stacking"
    reverse_model: component; hybrid_free_premium: component; stacked_trial: component; annual_prepay: component; bundled_service_product: component
    mastermind_layer: component; resell_model: component
  }
  three_ds_system: {
    shape: sql_table; class: diagnostic_table; label: "3 Ds System"
    document_sop: component; demonstrate_recorded: component; duplicate_qa_passed: component
  }
  performance_diamond: {
    shape: sql_table; class: key_table; label: "Performance Diamond"; style.fill: "#880E4F"; style.stroke: "#880E4F"
    hook_score: component; avatar_score: component; promise_score: component; price_score: component; weakest_corner_fix_first: component
  }
  referral_loop_math: {
    shape: sql_table; class: diagnostic_table; label: "Referral Loop Math"
    nps_question: component; promoters_ask_today: component; passives_ask_what_would_make_9: component; detractors_fix_first: component
  }
}

# Cross-book evolution pipeline
gym_launch.eight_step_offer -> offers.grand_slam_offer: "abstracted to" { style.stroke-width: 3 }
gym_launch.lead_scrambler -> leads.core_four: "formalized as" { style.stroke-width: 3 }
gym_launch.wedding_cake -> money.money_model: "abstracted to" { style.stroke-width: 3 }
gym_launch.closer -> offers.grand_slam_offer: "closes" { style.stroke-width: 3 }
offers.grand_slam_offer -> leads.core_four: "prerequisite" { style.stroke-width: 3 }
offers.value_equation -> leads.lead_value_equation: "variant of" { style.stroke-width: 3 }
leads.lead_magnet -> money.attraction_offers: "becomes" { style.stroke-width: 3 }
offers.grand_slam_offer -> money.attraction_offers: "Stage I of" { style.stroke-width: 3 }
leads -> money: "prerequisite" { style.stroke-width: 3 }
offers -> leads: "prerequisite" { style.stroke-width: 3 }

# Lost Chapters diagnostic feedback
lost.avatar_5x -> offers.tiny_market_big_money: "dx" { style.stroke-dash: 4; style.stroke-width: 2 }
lost.advanced_attraction -> money.attraction_offers: "dx" { style.stroke-dash: 4; style.stroke-width: 2 }
lost.ltgp_cac_ppd -> money.thirty_day_cash_cycle: "dx" { style.stroke-dash: 4; style.stroke-width: 2 }
lost.advanced_offer_stacking -> money.money_model: "dx" { style.stroke-dash: 4; style.stroke-width: 2 }
lost.performance_diamond -> offers.grand_slam_offer: "dx" { style.stroke-dash: 4; style.stroke-width: 2 }
lost.referral_loop_math -> leads.seven_referral_methods: "dx" { style.stroke-dash: 4; style.stroke-width: 2 }

legend: "Legend" {
  near: top-right
  grid-rows: 9
  style.fill: white
  style.stroke: "#78716c"
  style.font-color: "#1c1917"
  style.border-radius: 8
  gym: "Gym Launch Secrets" { shape: rectangle; style.fill: "#FFF8E1"; style.stroke: "#FFA000"; style.font-color: "#1c1917" }
  offer: "$100M Offers" { shape: rectangle; style.fill: "#EDE7F6"; style.stroke: "#5E35B1"; style.font-color: "#1c1917" }
  lead: "$100M Leads" { shape: rectangle; style.fill: "#E0F7FA"; style.stroke: "#00838F"; style.font-color: "#1c1917" }
  cash: "$100M Money Models" { shape: rectangle; style.fill: "#F1F8E9"; style.stroke: "#558B2F"; style.font-color: "#1c1917" }
  chapters: "$100M Lost Chapters" { shape: rectangle; style.fill: "#FCE4EC"; style.stroke: "#C2185B"; style.font-color: "#1c1917" }
  normal: "Normal framework" { shape: rectangle; style.fill: white; style.stroke: "#57534e"; style.font-color: "#1c1917" }
  key: "Key output" { shape: rectangle; style.fill: "#1c1917"; style.stroke: "#1c1917"; style.font-color: "#ffffff"; style.bold: true }
  diagnostic: "Diagnostic framework" { shape: rectangle; style.fill: white; style.stroke: "#C2185B"; style.stroke-dash: 4; style.font-color: "#1c1917" }
  lines: "Solid = pipeline · dashed = diagnostic" { shape: rectangle; style.fill: white; style.stroke: "#94A3B8"; style.stroke-dash: 2; style.font-color: "#1c1917" }
}
```

## [0446] 08-02 13:34

(2026-08-02T13:34:46)
confirm the meta workflow generator is still working with qwen3.5 9B Q4 thinking with at least half the the example prompts in terms of making workflows that if run would accomplish the prompts objectives.

## [0447] 08-02 13:35

(2026-08-02T13:35:23)
there should be over 20 prompts

## [0448] 08-02 13:38

(2026-08-02T13:38:23)
how do I run that model with llama.cpp with all those settings with one command where after starting up will be viewable in the browser?

## [0449] 08-03 08:37

(2026-08-03T08:37:33)
status?

## [0450] 08-03 16:52

(2026-08-03T16:52:01)
lets stop everything here then give me the full results and paths to them

