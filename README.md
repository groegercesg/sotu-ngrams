# State of the Union N-grams

The following repo implements a N-gram language model in Rust and uses it to generate synthetic exerpts from hypothetical Presidential "State of the Union" address. Below are some samples:

> And the whole world looks to us for protection.
>
> Industry is always necessary to keep economies in balance.
> 
> Everything is a possibility.
> 
> We will continue along the path toward a balanced budget.
>
> This Administration will be remembered for support of the workers who are not truly disabled.

*(Generated using the entire SOTU corpus, a Quad-gram model and the Probabilistic text generation mode.)*

## What is a "State of the Union"?

A "State of the Union" (SOTU) address is a speech delivered annually by the President of the United States to Congress. Typically, the president outlines their administration's achievements from the previous year and goals for the coming one. It's an important event, serving as a demonstration of the President's priorities and focus for the coming year.

The N-gram model uses 244 SOTU addresses from the American Presidency Project by [UC Santa Barbara](https://www.presidency.ucsb.edu/documents/presidential-documents-archive-guidebook/annual-messages-congress-the-state-the-union).

## N-Gram Language Models

N-gram language models are statistical models commonplace in NLP and lingustics settings. They use the frequency of words (grams) to predict future text. These rely on the independence assumption, that the probability of a word **only** depends on a fixed number of previous words (history).

This implementation allows the history size, the number of grams used, to be varied -hence it's an N-gram model. For this implementation the following sources have been heavily relied on:

- Speech and Language Processing. Daniel Jurafsky & James H. Martin, 2023. [Link](https://web.stanford.edu/~jurafsky/slp3/3.pdf)
- Foundations of Natural Language Processing, N-gram language models. Alex Lascarides, 2020. [Link](https://www.inf.ed.ac.uk/teaching/courses/fnlp/lectures/03_slides.pdf)

## Text Samples

The repository also includes the following sample texts, used for tests and debugging:

- `shakespeare_alllines.txt`: [Shakespeare Plays](https://www.kaggle.com/datasets/kingburrito666/shakespeare-plays?select=alllines.txt)
- `biden_sotu_2024.txt`: [Joe Biden State of the Union, 2024](https://www.whitehouse.gov/briefing-room/speeches-remarks/2024/03/07/remarks-of-president-joe-biden-state-of-the-union-address-as-prepared-for-delivery-2/)
- `biden_sotu_2022.txt`: [Joe Biden State of the Union, 2022](https://gist.github.com/fzliu/973bb1d659a740b1d78a659f90be4a02)
