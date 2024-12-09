use std::{cell::RefCell, collections::VecDeque, path::Path, rc::Rc};

use crate::Answer;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Block {
    digit: i64,
    count: i64,
}

fn build_blocks(input: &str) -> (Vec<Block>, Vec<i64>) {
    let mut gaps = Vec::new();
    let mut blocks = Vec::new();
    for (i, char) in input.chars().enumerate() {
        let digit = char.to_digit(10).map(|x| x as i64);
        if let Some(digit) = digit {
            if i % 2 == 0 {
                blocks.push(Block {
                    digit: i as i64 / 2,
                    count: digit,
                });
            } else {
                gaps.push(digit);
            }
        }
    }
    (blocks, gaps)
}

fn rebuild_blocks(blocks: Vec<Block>, gaps: Vec<i64>) -> Vec<Block> {
    let mut res = Vec::new();
    let mut gaps = VecDeque::from(gaps);
    let mut blocks = VecDeque::from(blocks);
    res.push(blocks.pop_front().unwrap());
    while !gaps.is_empty() {
        let gap = gaps.pop_front().unwrap();
        let block = blocks.pop_back().unwrap();
        match gap.cmp(&block.count) {
            std::cmp::Ordering::Greater => {
                gaps.push_front(gap - block.count);
                res.push(block);
                gaps.pop_back();
            }
            std::cmp::Ordering::Less => {
                let block_remain = block.count - gap;
                res.push(Block {
                    digit: block.digit,
                    count: gap,
                });
                if let Some(block) = blocks.pop_front() {
                    res.push(block)
                }
                blocks.push_back(Block {
                    digit: block.digit,
                    count: block_remain,
                });
            }
            std::cmp::Ordering::Equal => {
                res.push(block);
                res.push(blocks.pop_front().unwrap());
                gaps.pop_back();
            }
        }
    }
    res.extend(blocks);
    res
}

fn part_1(input: &str) -> Option<i64> {
    let (mut blocks, gaps) = build_blocks(input);
    blocks = rebuild_blocks(blocks, gaps);
    blocks
        .into_iter()
        .scan(0, |idx, Block { digit, count }| {
            let a1 = *idx;
            let an = *idx + count - 1;
            let sn = (a1 + an) * count / 2;
            *idx += count;
            Some(sn * digit)
        })
        .sum::<i64>()
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum BlockOrGap {
    Block(Block),
    Gap(i64),
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    data: BlockOrGap,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

fn build_bg(input: &str) -> (Rc<RefCell<Node>>, Vec<Rc<RefCell<Node>>>) {
    let head = Rc::new(RefCell::new(Node {
        data: BlockOrGap::Gap(0),
        next: None,
        prev: None,
    }));
    let mut v = vec![];
    let mut cur = head.clone();
    for (i, char) in input.chars().enumerate() {
        let digit = char.to_digit(10).map(|x| x as i64);
        if let Some(digit) = digit {
            if i % 2 == 0 {
                let block = Block {
                    digit: i as i64 / 2,
                    count: digit,
                };
                let node = Node {
                    data: BlockOrGap::Block(block),
                    next: None,
                    prev: Some(cur.clone()),
                };
                let rc_node = Rc::new(RefCell::new(node));
                cur.borrow_mut().next = Some(rc_node.clone());
                v.push(rc_node.clone());
                cur = rc_node;
            } else {
                let gap = BlockOrGap::Gap(digit);
                let node = Node {
                    data: gap,
                    next: None,
                    prev: Some(cur.clone()),
                };
                let rc_node = Rc::new(RefCell::new(node));
                cur.borrow_mut().next = Some(rc_node.clone());
                cur = rc_node;
            }
        }
    }
    let next = head.borrow().next.clone();
    (next.unwrap(), v)
}

fn part_2(input: &str) -> Option<i64> {
    let (head, v) = build_bg(input);
    for block_node in v.iter().rev() {
        let block = match block_node.borrow().data {
            BlockOrGap::Block(block) => block,
            _ => unreachable!(),
        };
        let mut maybe_curr = Some(head.clone());
        while let Some(curr) = maybe_curr {
            let data = curr.borrow_mut().data;
            match data {
                BlockOrGap::Block(curr_block) => {
                    if curr_block.digit == block.digit {
                        break;
                    }
                }
                BlockOrGap::Gap(gap) => {
                    match block.count.cmp(&gap) {
                        std::cmp::Ordering::Equal => {
                            std::mem::swap(
                                &mut curr.borrow_mut().data,
                                &mut block_node.borrow_mut().data,
                            );
                            break;
                        }
                        std::cmp::Ordering::Less => {
                            // split gap into two
                            let gap_next = curr.borrow().next.clone().unwrap();

                            let gap_split_node = Rc::new(RefCell::new(Node {
                                data: BlockOrGap::Gap(gap - block.count),
                                next: Some(gap_next.clone()),
                                prev: Some(curr.clone()),
                            }));
                            curr.borrow_mut().next = Some(gap_split_node.clone());
                            gap_next.borrow_mut().prev = Some(gap_split_node.clone());
                            curr.borrow_mut().data = BlockOrGap::Gap(block.count);
                            std::mem::swap(
                                &mut curr.borrow_mut().data,
                                &mut block_node.borrow_mut().data,
                            );

                            break;
                        }
                        std::cmp::Ordering::Greater => {}
                    }
                }
            }
            let next = curr.borrow().next.clone();
            maybe_curr = next;
        }
    }
    let mut maybe_cur = Some(head.clone());
    let mut idx = 0;
    let mut ans = 0;
    while let Some(cur) = maybe_cur {
        let data = cur.borrow().data;
        match data {
            BlockOrGap::Block(block) => {
                let Block { digit, count } = block;
                let a1 = idx;
                let an = idx + count - 1;
                let sn = (a1 + an) * count / 2;
                ans += sn * digit;
                idx += count;
            }
            BlockOrGap::Gap(gap) => {
                idx += gap;
            }
        }
        let next = cur.borrow().next.clone();
        maybe_cur = next;
    }
    ans.into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer::from_parts(part_1, part_2)
}
