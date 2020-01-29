pub mod from_serialized;
use json::{parse, JsonResult, JsonValue};
use std::collections::HashMap;

// Not exported - only here as sample code
// mod from_encoded;

use node_runtime::forum::InputValidationLengthConstraint;
use node_runtime::forum::{
    BlockchainTimestamp, Category, ChildPositionInParentCategory, ForumUser, ModerationAction,
    Moderator, Post, PostTextChange, Thread,
};

use node_runtime::{
    AccountId, BlockNumber, CategoryId, ForumConfig, ForumUserId, ModeratorId, Moment, PostId,
    ThreadId,
};

pub fn new_validation(min: u16, max_min_diff: u16) -> InputValidationLengthConstraint {
    return InputValidationLengthConstraint { min, max_min_diff };
}

// pub fn adapt_config() {
//     let forum_data = from_serialized::parse_forum_json();
//     if forum_data.is_err() {
//         println!("Can not read data from old forum module");
//     }
//     let data = forum_data.unwrap();
//     let forum_user_accounts = data.threads.iter().map(|thread| &thread.1.author_id);
// read all account from post.AccountId, thread.AccountId then register account for them
// read all account from ModerationAction and category, then register moderator for them
// adapt category, thread and post one by one
// }

pub fn read_json() -> JsonValue {
    let data = include_str!("../../res/forum_data_acropolis_serialized.json");

    let parsed_data = parse(data);
    if parsed_data.is_err() {
        println!("Can not parse forum data from json.");
    };
    parsed_data.unwrap()
}

pub fn parse_data(
    data: from_serialized::ForumData,
) -> (
    Vec<ForumUser<AccountId>>,
    Vec<Moderator<AccountId>>,
    Vec<Category<CategoryId, ThreadId, BlockNumber, Moment>>,
    Vec<Thread<ForumUserId, ModeratorId, CategoryId, BlockNumber, Moment>>,
    Vec<Post<ForumUserId, ModeratorId, ThreadId, BlockNumber, Moment>>,
) {
    let mut forum_users: Vec<ForumUser<AccountId>> = vec![];
    let mut moderators: Vec<Moderator<AccountId>> = vec![];
    let mut categories: Vec<Category<CategoryId, ThreadId, BlockNumber, Moment>> = vec![];
    let mut threads: Vec<Thread<ForumUserId, ModeratorId, CategoryId, BlockNumber, Moment>> =
        vec![];
    let mut posts: Vec<Post<ForumUserId, ModeratorId, ThreadId, BlockNumber, Moment>> = vec![];

    let mut forum_user_account_map: HashMap<[u8; 32], u64> = HashMap::new();
    let mut moderator_account_map: HashMap<[u8; 32], u64> = HashMap::new();

    let mut next_forum_user_id: u64 = 1;
    let mut next_moderator_id: u64 = 1;

    // read one by one get moderator
    for index in 0..data.categories.len() {
        let moderator_account_id = data.categories[index].1.moderator_id.clone();
        if moderator_account_map.contains_key(AsRef::<[u8; 32]>::as_ref(&moderator_account_id)) {
            moderator_account_map.insert(*moderator_account_id.as_ref(), next_moderator_id);
            next_moderator_id += 1;
        };
    }

    // read one by one get forum user account and moderator account
    for index in 0..data.threads.len() {
        let forum_user_account_id = data.threads[index].1.author_id.clone();
        if forum_user_account_map.contains_key(AsRef::<[u8; 32]>::as_ref(&forum_user_account_id)) {
            forum_user_account_map.insert(*forum_user_account_id.as_ref(), next_forum_user_id);
            next_forum_user_id += 1;
        };
        if data.threads[index].1.moderation.is_some() {
            let moderator_account_id = data.threads[index]
                .1
                .moderation
                .clone()
                .unwrap()
                .moderator_id
                .clone();
            if moderator_account_map.contains_key(AsRef::<[u8; 32]>::as_ref(&moderator_account_id))
            {
                moderator_account_map.insert(*moderator_account_id.as_ref(), next_moderator_id);
                next_moderator_id += 1;
            };
        }
    }

    // read each post and get forum user account

    // mapping category
    let categories = data
        .categories
        .iter()
        .map(|category| Category {
            id: category.1.id,
            title: category.1.title.clone(),
            description: category.1.description.clone(),
            created_at: BlockchainTimestamp {
                block: category.1.created_at.block,
                time: category.1.created_at.time,
            },
            deleted: category.1.deleted,
            archived: category.1.archived,
            num_direct_subcategories: category.1.num_direct_subcategories,
            num_direct_unmoderated_threads: category.1.num_direct_unmoderated_threads,
            num_direct_moderated_threads: category.1.num_direct_moderated_threads,
            position_in_parent_category: category.1.position_in_parent_category.as_ref().map(
                |value| ChildPositionInParentCategory {
                    parent_id: value.parent_id,
                    child_nr_in_parent_category: value.child_nr_in_parent_category,
                },
            ),
            sticky_thread_ids: vec![],
        })
        .collect();

    // mapping threads

    // mapping posts

    // build forum users from forum user account map

    // build moderators from moderator account map

    (forum_users, moderators, categories, threads, posts)
}

pub fn adapt_forum_config(
    forum_sudo: AccountId,
    // forum_data: from_serialized::ForumData,
) -> ForumConfig {
    ForumConfig {
        forum_user_by_id: vec![],
        next_forum_user_id: 1,
        moderator_by_id: vec![],
        next_moderator_id: 1,
        category_by_id: vec![],
        next_category_id: 1,
        thread_by_id: vec![],
        next_thread_id: 1,
        post_by_id: vec![],
        next_post_id: 1,

        forum_sudo: forum_sudo,
        category_by_moderator: vec![],
        max_category_depth: 5,
        reaction_by_post: vec![],

        category_title_constraint: InputValidationLengthConstraint {
            min: 10,
            max_min_diff: 140,
        },

        category_description_constraint: InputValidationLengthConstraint {
            min: 10,
            max_min_diff: 140,
        },

        thread_title_constraint: InputValidationLengthConstraint {
            min: 3,
            max_min_diff: 43,
        },

        post_text_constraint: InputValidationLengthConstraint {
            min: 1,
            max_min_diff: 1001,
        },

        thread_moderation_rationale_constraint: InputValidationLengthConstraint {
            min: 10,
            max_min_diff: 2000,
        },

        post_moderation_rationale_constraint: InputValidationLengthConstraint {
            min: 10,
            max_min_diff: 2000,
        }, // JUST GIVING UP ON ALL THIS FOR NOW BECAUSE ITS TAKING TOO LONG
        label_name_constraint: InputValidationLengthConstraint {
            min: 10,
            max_min_diff: 20,
        },
        poll_desc_constraint: InputValidationLengthConstraint {
            min: 10,
            max_min_diff: 200,
        },
        poll_items_constraint: InputValidationLengthConstraint {
            min: 4,
            max_min_diff: 20,
        },
        user_name_constraint: InputValidationLengthConstraint {
            min: 6,
            max_min_diff: 20,
        },
        user_self_introduction_constraint: InputValidationLengthConstraint {
            min: 10,
            max_min_diff: 200,
        },
        post_footer_constraint: InputValidationLengthConstraint {
            min: 10,
            max_min_diff: 140,
        },

        label_by_id: vec![],
        next_label_id: 1,
        category_labels: vec![],
        thread_labels: vec![],
        max_applied_labels: 5,
    }
}
