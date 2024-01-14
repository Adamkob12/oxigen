use super::*;

impl<'r, R: Resource> std::ops::Deref for Res<'r, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        downcast_res(self.lock.deref()).unwrap()
    }
}

impl<'r, R: Resource> std::ops::Deref for ResMut<'r, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        downcast_res(self.lock.deref()).unwrap()
    }
}

impl<'r, R: Resource> std::ops::DerefMut for ResMut<'r, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        downcast_res_mut(self.lock.deref_mut()).unwrap()
    }
}
