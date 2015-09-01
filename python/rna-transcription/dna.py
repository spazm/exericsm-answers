def to_rna(dna_seq):
    """
    Transcribes a DNA sequence to it's RNA complement,
    mapping G->C, C->G, T->A, A->U.

    Invalid characters in the dna_sequence will cause
    an Invalid_Sequence Exception
    """
    dna_to_rna = {'G': 'C',
                  'C': 'G',
                  'T': 'A',
                  'A': 'U'}

    output = ""
    for c in dna_seq:
        if c not in dna_to_rna:
            raise Invalid_Sequence(c)
        output += dna_to_rna[c]

    return output


class Invalid_Sequence(BaseException):
    pass
