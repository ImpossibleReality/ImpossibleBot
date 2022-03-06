import ray
import psutil
from transformers import GPT2Tokenizer, TFGPT2Model, set_seed

tokenizer = GPT2Tokenizer.from_pretrained('gpt2')
model = TFGPT2Model.from_pretrained('gpt2')


num_cpus = psutil.cpu_count(logical=True)
print('Number of available CPUs:', num_cpus)

# Start Ray cluster
ray.init(num_cpus=num_cpus, ignore_reinit_error=True)


set_seed(42)

model_id = ray.put(model)
tokenizer_id = ray.put(tokenizer)


@ray.remote
def inner_generate_text(input, model, tokenizer):
    encoded_input = tokenizer(input, return_tensors='tf')
    return model(encoded_input)


async def generate_text(input):
    return await inner_generate_text.remote(input, model_id, tokenizer_id)
