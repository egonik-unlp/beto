extern crate chemfiles;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Display};
use chemfiles::{Trajectory, Frame, Atom, AtomRef};
use petgraph::stable_graph::StableGraph;
use itertools::Itertools;
use petgraph::{IntoWeightedEdge, Graph};


struct AtomicWrapper<'a>(AtomRef<'a>);

struct BondedAtom<'b> {
    atom :AtomRef<'b>,
    pos : usize
}


impl<'a> BondedAtom<'a> {
    fn new(a : AtomRef, b : usize ) -> BondedAtom {
        BondedAtom { atom: a, pos: b }
    }
}

impl<'a> Hash for BondedAtom<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.atom.name().hash(state);
    }
}



impl<'a> Display for BondedAtom<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,  "Átomo de {} indice {} ",self.atom.name(), self.pos )
    }
}

impl<'a> Default for BondedAtom<'a> {
    fn default() -> Self {
        BondedAtom { atom: AtomRef {}, pos: 0 }
    }
}


impl<'a> Display for AtomicWrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,  "Átomo {} {} {} ", self.0.charge(), self.0.name(), self.0.mass() )
    }
}

impl<'a> PartialEq for BondedAtom<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }    
}

impl<'a> Eq for BondedAtom<'a> {} 


fn distribute_ref<T> ( i : &(T,T) ) -> (&T, &T) {
    (&i.0, &i.1)
} 




fn main() {
    let mut trajectory = Trajectory::open("mof.pdb", 'r').unwrap();
    let mut frame = Frame::new();

    trajectory.read(&mut frame).unwrap();

    println!("There are {} atoms in the frame", frame.size());

    let positions = frame.positions();
    let topology = frame.topology();
    println!("tengo {} residuos", topology.residues_count());
    
    // TCPP
    let mut trajectory = Trajectory::open("tcpp.pdb", 'r').unwrap();
    let mut frame = Frame::new();

    trajectory.read(&mut frame).unwrap();

    println!("There are {} atoms in the frame", frame.size());

 
    println!("tengo {} residuos", topology.residues_count());
    let tcpp = frame.topology().residue(0).unwrap();
    let topo = frame.topology();
    let bonds = frame.topology().bonds()
                .into_iter()
                .map(|[a,b]| {
                                        (BondedAtom::new(topo.atom(a), b ), BondedAtom::new( topo.atom(b),b )) 
                })
                .collect::<Vec<_>>();
    for bond in &bonds {
        println!("idx {} car {} -bonds-  idx {} car {}",bond.0.pos, bond.0.atom.name(), bond.1.pos, bond.1.atom.name()); 
    }
   let mut TCPP = Graph::<BondedAtom, BondedAtom>::new();
    


    let (mut l, mut  r) : (Vec<(_)>, Vec<(_)>) = bonds.iter().map(distribute_ref).unzip(); 
    l.append(&mut r);
    let nodes : Vec<&BondedAtom> = l.into_iter().unique().collect();

    TCPP.extend_with_edges(bonds.iter().map(|(at1, at2)| (at1.pos,at2.pos)  ).collect())
    
    
    // let gbonds = frame.topology().bonds()
    //             .into_iter()
    //             .map(| [a,b] | (a, b) )
    //             .collect::<Vec<_>>();

    // let pg = StableGraph::<_,_>::from_edges(gbonds);        





    //for bond in frame
    //            .topology()
    //            .bonds() {
    //                println!("{:?}", bond );
 //  
 //                   let ea = match bond {
 //                       [a,b] => EnhancedAtom::new(topo.atom(a).clone(), vec![topo.atom(b).clone()]),
 //                       _ => panic!("At the disco")
 //                   }; 
 //                   println!("{}", ea);
 //                   
//                }
//    
 //   
    // Do awesome things with the positions here !
}
