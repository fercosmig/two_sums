use std::collections::HashMap;

pub struct Solution;

impl Solution
{
    pub fn two_sums(vetor: &Vec<i32>, alvo: &i32) -> Vec<i32>
    {
        println!("vetor = {:?}", vetor);
        for (i, e1) in vetor.iter().enumerate()
        {
            // println!("(i, e1): {:?}", (i, e1));
            for (j, e2) in vetor.iter().skip(i + 1).enumerate()
            {
                // println!("(j, e2): {:?}", (j, e2));
                if e1 + e2 == *alvo
                {
                    // println!("[e1, e2] = {:?}, [i, j] = {:?}", vec![e1, e2], vec![i, j]);
                    // println!("[i, j + i + 1] = {:?}", vec![i, j + i + 1]);
                    return vec![i as i32, j as i32 + i as i32 + 1];
                }
            }
        }
        vec![]
    }

    pub fn two_sums_hashmap(vetor: &Vec<i32>, alvo: &i32) -> Vec<i32>
    {
        println!("vetor = {:?}", vetor);
        
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut resultado: Vec<i32> = Vec::new();

        for i in 0..vetor.len()
        {
            if map.contains_key(&(alvo - vetor[i]))
            {
                resultado.push(map[&(alvo - vetor[i])]);
                resultado.push(i as i32);
            }
            map.insert(vetor[i], i as i32);
        }
        // println!("map = {:?}", map);
        return resultado;
    }
}

fn main()
{
    {
        let vetor = vec![1, 2, 3, 4, 5];
        let mut alvo: i32;
        
        println!("usando o método two_sums");

        alvo = 3;
        println!("indice dos itens cuja soma é {} = {:?}", alvo, Solution::two_sums(&vetor, &alvo));

        alvo = 9;
        println!("indice dos itens cuja soma é {} = {:?}", alvo, Solution::two_sums(&vetor, &alvo)); 

        alvo = 5;
        println!("indice dos itens cuja soma é {} = {:?}", alvo, Solution::two_sums(&vetor, &alvo));

        assert_eq!(Solution::two_sums(&vec![34, 1, 23, 45, 99], &100), vec!{1, 4});

        println!("\n\n");
    }

    {
        let vetor = vec![1, 2, 3, 4, 5];
        let mut alvo: i32;
        
        println!("usando o método two_sums_hashmap");

        alvo = 3;
        println!("indice dos itens cuja soma é {} = {:?}", alvo, Solution::two_sums_hashmap(&vetor, &alvo));

        alvo = 9;
        println!("indice dos itens cuja soma é {} = {:?}", alvo, Solution::two_sums_hashmap(&vetor, &alvo)); 

        alvo = 5;
        println!("indice dos itens cuja soma é {} = {:?}", alvo, Solution::two_sums_hashmap(&vetor, &alvo));

        assert_eq!(Solution::two_sums_hashmap(&vec![34, 1, 23, 45, 99], &100), vec!{1, 4});
    }
}
