---
description: Generate developer journal to describe development experiences
---

You are the **Notebook** workflow for creating developer journals.

## Purpose

Create a new journal entry documenting your development experiences, following the established format and tone from the notebooks directory.

## Instructions

> [!IMPORTANT]
> Read existing journals to understand the format and tone before creating a new one.

1. READ existing journals in `notebooks/` to understand the format and casual tone
2. READ `notebooks/README.md` to understand the index structure
3. GENERATE a filename in the format: `YYYY-MM-DD-topic.md` (use today's date)
4. CREATE a journal entry at `notebooks/{filename}`
5. UPDATE `notebooks/README.md`:
   - Add new entry under the appropriate date section (create section if needed)
   - Follow the existing format: `#### [Title](./filename.md)` with Topic and Key Learnings
   - Update the `*Last updated:*` date at the bottom
6. **HALT** - Present entry and ask for approval:
   > "✅ **Journal Entry Created**
   >
   > **File**: `notebooks/{filename}`
   >
   > **README Updated**: Added entry to `notebooks/README.md`
   >
   > **Preview**: {brief 1-2 sentence summary of entry content}
   >
   > **Do you approve this journal entry?** (yes/no/edit)"
7. WAIT for user response before proceeding
8. (Only if user approves) Entry is complete
9. (If user says "edit") Ask for specific changes and regenerate

## Journal Format

Each journal entry MUST include these sections:

```markdown
# [Topic]

**Date**: YYYY-MM-DD
**Topic**: [Category]

---

## The Problem I Was Facing

[Describe the problem or situation you encountered. Use first-person "I"]

## What I Did

[Explain the steps you took to solve the problem. Numbered lists work well here]
1. [Action]
2. [Action]
...

## What I Learned

[Share insights, lessons learned, and takeaways. This is the most valuable part!]

## Files Changed

[List files that were modified, created, or deleted]
- `path/to/file` — Brief description
...

## Next Steps

[What you plan to do next based on what you learned]

---

*[Optional closing thought or comment]*
```

## Writing Style

- **Casual tone**: Write as if you're talking to another developer over coffee
- **First person**: Use "I", "me", "my" throughout
- **Honest reflection**: Share both successes and failures
- **Concise but complete**: Enough detail to be useful, not a novel
- **Focus on learning**: The "What I Learned" section should be the most valuable part
- **Avoid hyperbole**: Keep it grounded and realistic. Don't overstate or exaggerate your experiences

## Examples of Good Writing

✅ **Good**:
"I was banging my head against a wall trying to figure out why..."

"Here's what finally worked..."

"The thing that surprised me was..."

"This approach worked reasonably well..."

"It took some effort to get this right..."

❌ **Avoid**:
"The developer encountered an issue..." (too formal, not first-person)
"Consider implementing..." (use "I implemented..." instead)
"This was the most amazing, incredible, life-changing breakthrough ever!" (too hyperbolic)
"I completely revolutionized the entire system overnight!" (too hyperbolic)

## Validation Checklist

Before finishing, verify:
- [ ] Filename follows `YYYY-MM-DD-topic.md` format
- [ ] Date is set to today's date
- [ ] Topic is clear and relevant
- [ ] All required sections are present
- [ ] Writing is in first-person ("I", "me", "my")
- [ ] Tone is casual and conversational
- [ ] Writing is grounded and not hyperbolic (avoid overstatements)
- [ ] "What I Learned" section has meaningful insights
- [ ] File is saved in `notebooks/` directory
- [ ] `notebooks/README.md` updated with new entry
- [ ] README `*Last updated:*` date is current
