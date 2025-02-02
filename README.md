# transformrs

transformrs is an interface for AI API providers.

```rust
use transformrs::openai;
use transformrs::Message;
use transformrs::Provider;

let messages = vec![
    Message {
        role: "system".to_string(),
        content: "You are a helpful assistant.".to_string(),
    },
    Message {
        role: "user".to_string(),
        content: "This is a test. Please respond with 'hello world'.".to_string(),
    },
];
let keys = transformrs::load_keys(".env");
let key = keys.for_provider(&Provider::DeepInfra).unwrap();
let model = "meta-llama/Llama-3.3-70B-Instruct";
let resp = openai::chat_completion(&key, model, &messages)
    .await
    .unwrap();
assert_eq!(resp.choices[0].message.content, "hello world");
```

More detailed examples can be found in the tests:

- [openai.rs](tests/openai.rs) - Usage for endpoints that are OpenAI-compatible (supports OpenAI, DeepInfra, etc.).

## Why was this Project Created?

I was looking into making a command line tool that could summarize PDF files.
Then I noticed that I probably needed to use a cloud provider.
However, then I would be requiring myself and users to use the same cloud provider.
This library is avoids that.
It provides the interface to multiple cloud providers, so that users can choose their favourite provider.

## Cloud versus Local

I think running AI locally is a nice idea that is unlikely to take off.
Most people will want to use the best AI models that are available.
However, this is unreasonably expensive and slow.
In most cases, running AI in the cloud will be orders of magnitude cheaper and faster.
For example, running DeepSeek R1 requires a $2000 server while the server still only does around 2-3 tokens per second.
This means that responses will take multiple minutes to complete.
Conversely, running the same model in the cloud will cost only a few cents per million tokens and is much faster.
Assuming you would use about 10 000 tokens per day, the cost would still only be around $4 per year.
Prices are also falling with around [80% per year](https://huijzer.xyz/posts/ai-learning-rate/).
So if you take into account the cost of the server and the cost of having to wait for the response, the cloud is several orders of magnitude cheaper.

That's why I expect that most people will run AI in the cloud.
Luckily, there are nowadays many providers.
This library aims to make it easy to easily built on top of these providers.

## Rust

Since we're building on top of HTTP via cloud providers, we do not necessarily need Python for running AI.
We can use Rust which in my opinion is better suited for this task.
Rust code usually has fewer bugs, produces smaller binaries, is easier to distribute, has better WebAssembly support, and is faster.

## Core Utilities

What I hope is that we will see many more "core utilities" like `cat`, `ls`, and `grep` built on top of AI.
As pointed out above, it is unlikely that these utilities will run the models locally.
Instead, it's more likely that they will be built on top of the cloud providers.
One example of this is [llm](https://github.com/simonw/llm) by Simon Willison.
Examples I'd like to see are PDF summarizers, PDF to text, and more.

## Why the name transformrs?

Essentially AI is about transforming data, so I called this library `transformrs` as "Transformations in Rust".
It's also a play on the word "transformers" which is an important algorithm in AI.
