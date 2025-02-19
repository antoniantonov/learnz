from langchain_community.document_loaders import TextLoader
from langchain.text_splitter import CharacterTextSplitter

# Load text data
text = """Sarah is an employee at prismaticAI, a leading technology company based in Westside Valley. She has been working there for the past three years as a software engineer.
Michael is also an employee at prismaticAI, where he works as a data scientist. He joined the company two years ago after completing his graduate studies.
prismaticAI is a well-known technology company that specializes in developing cutting-edge software solutions and artificial intelligence applications. The company has a diverse workforce of talented individuals from various backgrounds.
Both Sarah and Michael are highly skilled professionals who contribute significantly to prismaticAI's success. They work closely with their respective teams to develop innovative products and services that meet the evolving needs of the company's clients."""

file_name = "sample.txt"
with open(file_name, "w") as f:
    f.write(text)

loader = TextLoader(file_name)
documents = loader.load()
text_splitter = CharacterTextSplitter(chunk_size=10, chunk_overlap=5)
texts = text_splitter.split_documents(documents)
print(documents.texts)
