mmd header: {{../templates/header.html}}
mmd footer: {{../templates/footer.html}}
css: /css/main.css
tags: ai vibe coding notes


# Process over Magic: Beyond Vibe Coding - Notes

It's a short book (100 or so pages) with video content as well. I actually
like short books. Less fluff, more stuff, as they say. Some of the tips are
pretty standard practices. Experienced developers start with a clean git, passing
test suite, and only commits working code. However, it's even more critical
these practices are followed when working with AI.

My school computer room had a poster up on the wall that read, "To err is human,
to really foul up requires a computer". This is even truer with AI. It allows you
amplify your progress, or your mistakes exponentially.

## Practices to follow when working with AI

* Guide the AI assistant, don't convince it
* Start with what you want the result to look like
* Use formats like Markdown, JSON or roles
* Use examples or exclusions to help narrow focus
* Iterate: plan, test, refine, compare, reuse prompts that work
* One prompt, one commit
* Only commit working code
* Always review code before committing
* Focused prompts - one of: Add a new feature, fix a bug, refactor code
* Stuck? Roll back. Don't waste time fighting it. Try a different approach
* Prefer being precise with your prompts over role based (e.g. "You're an expert
  developer" tries to make the AI an actor which it's not)
* Don't be vague (e.g. "be careful"). Instead, use "Always run tests and verify
  they pass before committing code".
* Remember to ask the AI to go back over the code (once committed) and check
  for refactoring opportunities (like you would do normally)
* When working on a large feature or kicking off new project, a good prompt
  is a clear and detailed document (1000 words or less),
  generally written in a document that can be referenced later, particularly
  if it's something complex that might need to be broken down further
  * Add it as a plan document
  * Shape it with iterations (this can take several hours)
  * Rewrite the document in a more compact format
* Walking Skeleton/Minimum Viable Product (MVP) is a good way to get started
  * Start with a simpler version of the feature
  * Add it to the plan document
  * Refactor it to be more efficient
* Take regular breaks to help maintain a sustainable, comfortable pace
* Ask the assistant to update the design document with all recent progress
* When working in large codebases
  * It's better to move in small steps
  * > You are a junior developer doing your best. If an instruction seems
    > unclear, wrong or unfamiliar, ask for more information before continuing
    [TODO: Check whether "You are a junior developer is actually necessary"]
  * Write pseudocode directly into a file and ask the AI to turn it into code
  * Try asking the AI to tell us how to do something without writing the code,
    then we write it ourselves. Learn by doing, guided by the AI. Ask the AI if
    you get stuck
* Debugging planner. Ask the AI to create a step-by-step debugging plan that
  follows progressive logic: starting with most likely cause and eliminating
  them one by one
* Parallel designing - Ask the AI to write or rewrite a feature in different
  ways. Explore design variations or different libraries. Review, tweak,
  rollback and try again. Think: AI guided spikes
* Use git worktrees
* Ask the AI to write a script to make some transformations much more efficient
  than asking directly (though Claude is quite good at doing this automatically)
* When working on one thing and we notice that something else is broken, commit
  the code (if it's working & passing tests) then immediately fix the broken
  thing and follow up with another commit.


I've used something similar to the following before. It's a good reminder that
with the right prompts, the AI can be extremely helpful and also not go blazing
ahead with the wrong idea.

* > If instructions are unclear, ask follow up questions before continuing
* > If a request is impossible, explain why and suggest an alternatives


## Things to watch out for

* Don't be fooled by confident responses
* Always review, verify and ensure adequate test coverage
* AI's don't know how to write code, they just know patterns
* AI's may write bad or poorly performing code, again, iterate on it

## Author Preferences

* Prefers to stop the assistant after 5 minutes if it hasn't produced a good
  result. There are exceptions to this, such as initial scaffolding

## Links

* [Process over Magic: Beyond Vibe Coding (Pragmatic Programmers)](https://pragprog.com/titles/ubaidev/process-over-magic-beyond-vibe-coding/)
