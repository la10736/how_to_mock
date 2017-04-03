#[derive(Debug, PartialEq, Clone)]
enum Actions {
    AnAction(u32),
    Nothing
}

trait ActionAdapter {
    fn process(&self, act: Actions);
}

struct ClassUnderTest<A>
    where A: ActionAdapter
{
    adapter : A
}

impl<A> ClassUnderTest<A>
    where A: ActionAdapter
{
    pub fn do_it(&self, actions: Vec<Actions>) {
        for a in actions.into_iter() {
            self.adapter.process(a)
        }
    }
}

fn main() {
    struct LogAdapter {};

    impl ActionAdapter for LogAdapter {
        fn process(&self, act: Actions) {
            println!("Process {:?}", act);
        }
    }

    let c = ClassUnderTest{ adapter: LogAdapter{} };

    c.do_it(vec!(Actions::AnAction(431), Actions::Nothing, Actions::AnAction(21)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    struct MockAdapter {
        processed: Vec<Actions>
    }

    impl ActionAdapter for MockAdapter {
        fn process(&self, act: Actions) {
            self.as_mut().processed.push(act);
        }
    }

    impl MockAdapter {
        fn new() -> Self {
            return MockAdapter{ processed: vec!() }
        }
        fn as_mut(&self) -> &mut Self {
            unsafe {&mut *(self as *const Self as *mut Self) }
        }
    }

    impl <A> ActionAdapter for std::rc::Rc<A>
        where A: ActionAdapter {
        fn process(&self, act: Actions) {
            self.deref().process(act)
        }
    }

    #[test]
    fn base() {
        let ad = std::rc::Rc::new(MockAdapter::new());
        let c = ClassUnderTest{ adapter: ad.clone() };
        let actions = vec!(Actions::AnAction(431), Actions::Nothing, Actions::AnAction(21));

        c.do_it(actions.clone());

        assert_eq!(ad.processed, actions);
    }
}