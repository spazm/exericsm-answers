from collections import Counter

def word_count(corpus):
    """ Count words in a corpus """

    count = Counter()
    for word in corpus.split():
        count[word] += 1

    return count
