use cge::gene::Gene;
use cge::gene::GeneExtras::*;

use utils::Individual;

// Percentage threshold for determining if fitness is "almost the same"
const SIMILAR_FITNESS_THRESHOLD: f64 = 0.01;

pub enum Category {
    Duplicate,
    Similar,
    Okay
}

pub fn compare(a: &Individual, b: &Individual) -> Category {
    let min = vec![a.network.size, b.network.size].iter().min().unwrap() + 1;
    let mut is_duplicate = true;

    for i in 0..min {
        let gene_a = variant(&a.network.genome[i]);
        let gene_b = variant(&b.network.genome[i]);

        if gene_a != gene_b {
            is_duplicate = false;
            break;
        }
    }

    if is_duplicate {
        Category::Duplicate
    } else {
        let mut neurons_a = Vec::new();
        let mut neurons_b = Vec::new();

        for i in 0..min {
            let gene_a = &a.network.genome[i];
            let gene_b = &b.network.genome[i];

            if let Neuron(_, _) = gene_a.variant {
                neurons_a.push(gene_a.id);
            }

            if let Neuron(_, _) = gene_b.variant {
                neurons_b.push(gene_b.id);
            }
        }

        if neurons_a == neurons_b {
            Category::Similar
        } else {
            Category::Okay
        }
    }
}

pub fn compare_fitness(a: &Individual, b: &Individual) -> bool {
    let diff = (a.fitness - b.fitness).abs();

    diff / a.fitness < SIMILAR_FITNESS_THRESHOLD
}

fn variant(gene: &Gene) -> i32 {
    match gene.variant {
        Neuron(_, _) => 0,
        Input(_) => 1,
        Forward => 2,
        Recurrent => 3,
        Bias => 4
    }
}