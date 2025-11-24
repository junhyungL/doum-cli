// 메뉴 구조 및 네비게이션 시스템

/// 메뉴 아이템
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub description: String,
}

impl MenuItem {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: description.into(),
        }
    }

    /// 표시용 포맷 (선택 리스트에 표시)
    pub fn display_label(&self) -> String {
        format!("{:12} - {}", self.label, self.description)
    }
}

/// 메뉴 컨텍스트 (네비게이션 상태 관리)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuAction {
    Continue, // 현재 메뉴 계속
    Back,     // 이전 메뉴로
    Exit,     // 프로그램 종료
}

/// 메뉴 빌더
pub struct MenuBuilder {
    items: Vec<MenuItem>,
    title: String,
    include_back: bool,
    include_exit: bool,
}

impl MenuBuilder {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            items: Vec::new(),
            title: title.into(),
            include_back: false,
            include_exit: true,
        }
    }

    pub fn with_back(mut self) -> Self {
        self.include_back = true;
        self
    }

    pub fn with_exit(mut self, include: bool) -> Self {
        self.include_exit = include;
        self
    }

    pub fn add_item(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.items.push(MenuItem::new(id, label, description));
        self
    }

    pub fn build(mut self) -> Menu {
        let mut all_items = Vec::new();

        // Add back option
        if self.include_back {
            all_items.push(MenuItem::new("back", "Back", "Back to previous menu"));
        }

        // Add user items
        all_items.append(&mut self.items);

        // Add exit option
        if self.include_exit {
            all_items.push(MenuItem::new("exit", "Exit", "Exit the program"));
        }

        Menu {
            title: self.title,
            items: all_items,
        }
    }
}

/// 메뉴
pub struct Menu {
    pub title: String,
    pub items: Vec<MenuItem>,
}

impl Menu {
    pub fn builder(title: impl Into<String>) -> MenuBuilder {
        MenuBuilder::new(title)
    }

    /// 메뉴 아이템 ID로 찾기
    pub fn get_item(&self, id: &str) -> Option<&MenuItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// 표시용 옵션 리스트 생성
    pub fn get_display_options(&self) -> Vec<String> {
        self.items.iter().map(|item| item.display_label()).collect()
    }

    /// 선택된 인덱스로부터 MenuItem 가져오기
    pub fn get_item_by_index(&self, index: usize) -> Option<&MenuItem> {
        self.items.get(index)
    }
}
