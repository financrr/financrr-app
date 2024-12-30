use loco_rs::controller::AppRoutes;
use loco_rs::prelude::Routes;

pub struct ExtendedAppRoutes {
    prefix: Option<String>,
    internal_app_routes: AppRoutes,
}

impl ExtendedAppRoutes {
    pub fn empty() -> Self {
        Self {
            prefix: None,
            internal_app_routes: AppRoutes::empty(),
        }
    }

    pub fn prefix(mut self, prefix: &str) -> Self {
        match self.prefix {
            None => self.prefix = Some(prefix.to_string()),
            Some(mut old_prefix) => {
                old_prefix.push_str(prefix);
                self.prefix = Some(old_prefix.to_string())
            }
        }

        self
    }

    pub fn reset_prefix(mut self) -> Self {
        self.prefix = None;

        self
    }

    pub fn add_route(mut self, mut routes: Routes) -> Self {
        let routes_prefix = {
            if let Some(mut prefix) = self.prefix.clone() {
                let routes_prefix = routes.prefix.clone().unwrap_or("".to_string());

                prefix.push_str(routes_prefix.as_str());
                Some(prefix)
            } else {
                routes.prefix.clone()
            }
        };

        if let Some(prefix) = routes_prefix {
            routes = routes.prefix(prefix.as_str());
        }

        self.internal_app_routes = self.internal_app_routes.add_route(routes);

        self
    }
}

impl From<ExtendedAppRoutes> for AppRoutes {
    fn from(value: ExtendedAppRoutes) -> Self {
        value.internal_app_routes
    }
}
