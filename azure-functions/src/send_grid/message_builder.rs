use crate::{
    bindings::SendGridMessage,
    send_grid::{Attachment, Content, EmailAddress, Personalization},
};
use serde_json::Value;

/// Represents a builder for SendGrid messages.
#[derive(Default)]
pub struct MessageBuilder(SendGridMessage);

impl MessageBuilder {
    /// Creates a new message builder.
    pub fn new() -> MessageBuilder {
        MessageBuilder(SendGridMessage::default())
    }

    /// Appends the given "to" email address to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().to("foo@example.com").build();
    /// assert_eq!(message.personalizations[0].to[0].email, "foo@example.com");
    /// assert_eq!(message.personalizations[0].to[0].name, None);
    /// ```
    pub fn to<T>(mut self, email: T) -> MessageBuilder
    where
        T: Into<String>,
    {
        self.append_to(EmailAddress::new(email));
        self
    }

    /// Appends the given "to" email address to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().to_with_name("foo@example.com", "Peter").build();
    /// assert_eq!(message.personalizations[0].to[0].email, "foo@example.com");
    /// assert_eq!(message.personalizations[0].to[0].name, Some("Peter".to_owned()));
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn to_with_name<T, U>(mut self, email: T, name: U) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.append_to(EmailAddress::new_with_name(email, name));
        self
    }

    /// Appends the given "to" email addresses to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// use azure_functions::send_grid::EmailAddress;
    /// let message = MessageBuilder::new().tos(
    ///     vec![
    ///         EmailAddress::new("foo@example.com"),
    ///         EmailAddress::new_with_name("bar@example.com", "Peter"),
    ///     ]).build();
    /// assert_eq!(message.personalizations[0].to[0].email, "foo@example.com");
    /// assert_eq!(message.personalizations[0].to[0].name, None);
    /// assert_eq!(message.personalizations[0].to[1].email, "bar@example.com");
    /// assert_eq!(message.personalizations[0].to[1].name, Some("Peter".to_owned()));
    /// ```
    pub fn tos<T>(mut self, addresses: T) -> MessageBuilder
    where
        T: IntoIterator<Item = EmailAddress>,
    {
        self.append_personalization();
        self.0.personalizations[0].to.extend(addresses);
        self
    }

    /// Appends the given "cc" email address to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().cc("foo@example.com").build();
    /// assert_eq!(message.personalizations[0].cc[0].email, "foo@example.com");
    /// assert_eq!(message.personalizations[0].cc[0].name, None);
    /// ```
    pub fn cc<T>(mut self, email: T) -> MessageBuilder
    where
        T: Into<String>,
    {
        self.append_cc(EmailAddress::new(email));
        self
    }

    /// Appends the given "cc" email address to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().cc_with_name("foo@example.com", "Peter").build();
    /// assert_eq!(message.personalizations[0].cc[0].email, "foo@example.com");
    /// assert_eq!(message.personalizations[0].cc[0].name, Some("Peter".to_owned()));
    /// ```
    pub fn cc_with_name<T, U>(mut self, email: T, name: U) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.append_cc(EmailAddress::new_with_name(email, name));
        self
    }

    /// Appends the given "cc" email addresses to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// use azure_functions::send_grid::EmailAddress;
    /// let message = MessageBuilder::new().ccs(
    ///     vec![
    ///         EmailAddress::new("foo@example.com"),
    ///         EmailAddress::new_with_name("bar@example.com", "Peter"),
    ///     ]).build();
    /// assert_eq!(message.personalizations[0].cc[0].email, "foo@example.com");
    /// assert_eq!(message.personalizations[0].cc[0].name, None);
    /// assert_eq!(message.personalizations[0].cc[1].email, "bar@example.com");
    /// assert_eq!(message.personalizations[0].cc[1].name, Some("Peter".to_owned()));
    /// ```
    pub fn ccs<T>(mut self, addresses: T) -> MessageBuilder
    where
        T: IntoIterator<Item = EmailAddress>,
    {
        self.append_personalization();
        self.0.personalizations[0].cc.extend(addresses);
        self
    }

    /// Appends the given "bcc" email address to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().bcc("foo@example.com").build();
    /// assert_eq!(message.personalizations[0].bcc[0].email, "foo@example.com");
    /// assert_eq!(message.personalizations[0].bcc[0].name, None);
    /// ```
    pub fn bcc<T>(mut self, email: T) -> MessageBuilder
    where
        T: Into<String>,
    {
        self.append_bcc(EmailAddress::new(email));
        self
    }

    /// Appends the given "bcc" email address to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().bcc_with_name("foo@example.com", "Peter").build();
    /// assert_eq!(message.personalizations[0].bcc[0].email, "foo@example.com");
    /// assert_eq!(message.personalizations[0].bcc[0].name, Some("Peter".to_owned()));
    /// ```
    pub fn bcc_with_name<T, U>(mut self, email: T, name: U) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.append_bcc(EmailAddress::new_with_name(email, name));
        self
    }

    /// Appends the given "bcc" email addresses to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// use azure_functions::send_grid::EmailAddress;
    /// let message = MessageBuilder::new().bccs(
    ///     vec![
    ///         EmailAddress::new("foo@example.com"),
    ///         EmailAddress::new_with_name("bar@example.com", "Peter"),
    ///     ]).build();
    /// assert_eq!(message.personalizations[0].bcc[0].email, "foo@example.com");
    /// assert_eq!(message.personalizations[0].bcc[0].name, None);
    /// assert_eq!(message.personalizations[0].bcc[1].email, "bar@example.com");
    /// assert_eq!(message.personalizations[0].bcc[1].name, Some("Peter".to_owned()));
    /// ```
    pub fn bccs<T>(mut self, addresses: T) -> MessageBuilder
    where
        T: IntoIterator<Item = EmailAddress>,
    {
        self.append_personalization();
        self.0.personalizations[0].bcc.extend(addresses);
        self
    }

    /// Sets the subject for the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().subject("hello world!").build();
    /// assert_eq!(message.personalizations[0].subject, Some("hello world!".to_owned()));
    /// ```
    pub fn subject<T>(mut self, subject: T) -> MessageBuilder
    where
        T: Into<String>,
    {
        self.append_personalization();
        self.0.personalizations[0].subject = Some(subject.into());
        self
    }

    /// Appends a header to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().header("foo", "bar").build();
    /// assert_eq!(message.personalizations[0].headers.get("foo").map(String::as_str), Some("bar"));
    /// ```
    pub fn header<T, U>(mut self, key: T, value: U) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.append_personalization();
        self.0.personalizations[0]
            .headers
            .insert(key.into(), value.into());
        self
    }

    /// Appends multiple headers to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// # use std::collections::HashMap;
    /// let mut headers = HashMap::new();
    /// headers.insert("foo".to_owned(), "bar".to_owned());
    /// headers.insert("bar".to_owned(), "baz".to_owned());
    /// let message = MessageBuilder::new().headers(headers).build();
    /// assert_eq!(message.personalizations[0].headers.get("foo").map(String::as_str), Some("bar"));
    /// assert_eq!(message.personalizations[0].headers.get("bar").map(String::as_str), Some("baz"));
    /// ```
    pub fn headers<T>(mut self, headers: T) -> MessageBuilder
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.append_personalization();
        self.0.personalizations[0].headers.extend(headers);
        self
    }

    /// Appends a substitution to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().substitution("foo", "bar").build();
    /// assert_eq!(message.personalizations[0].substitutions.get("foo").map(String::as_str), Some("bar"));
    /// ```
    pub fn substitution<T, U>(mut self, key: T, value: U) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.append_personalization();
        self.0.personalizations[0]
            .substitutions
            .insert(key.into(), value.into());
        self
    }

    /// Appends multiple substitutions to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// # use std::collections::HashMap;
    /// let mut substitutions = HashMap::new();
    /// substitutions.insert("foo".to_owned(), "bar".to_owned());
    /// substitutions.insert("bar".to_owned(), "baz".to_owned());
    /// let message = MessageBuilder::new().substitutions(substitutions).build();
    /// assert_eq!(message.personalizations[0].substitutions.get("foo").map(String::as_str), Some("bar"));
    /// assert_eq!(message.personalizations[0].substitutions.get("bar").map(String::as_str), Some("baz"));
    /// ```
    pub fn substitutions<T>(mut self, substitutions: T) -> MessageBuilder
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.append_personalization();
        self.0.personalizations[0]
            .substitutions
            .extend(substitutions);
        self
    }

    /// Sets the template data for the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// use serde_json::{json, to_string};
    /// let message = MessageBuilder::new().template_data(json!({ "foo": "bar" })).build();
    /// assert_eq!(to_string(message.personalizations[0].template_data.as_ref().unwrap()).unwrap(), r#"{"foo":"bar"}"#);
    /// ```
    pub fn template_data(mut self, data: Value) -> MessageBuilder {
        if let Value::Object(map) = data {
            self.append_personalization();
            self.0.personalizations[0].template_data = Some(map);
        } else {
            panic!("template data must be a JSON object");
        }

        self
    }

    /// Appends a custom argument to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().custom_arg("foo", "bar").build();
    /// assert_eq!(message.personalizations[0].custom_args.get("foo").map(String::as_str), Some("bar"));
    /// ```
    pub fn custom_arg<T, U>(mut self, key: T, value: U) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.append_personalization();
        self.0.personalizations[0]
            .custom_args
            .insert(key.into(), value.into());
        self
    }

    /// Appends multiple custom arguments to the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// # use std::collections::HashMap;
    /// let mut args = HashMap::new();
    /// args.insert("foo".to_owned(), "bar".to_owned());
    /// args.insert("bar".to_owned(), "baz".to_owned());
    /// let message = MessageBuilder::new().custom_args(args).build();
    /// assert_eq!(message.personalizations[0].custom_args.get("foo").map(String::as_str), Some("bar"));
    /// assert_eq!(message.personalizations[0].custom_args.get("bar").map(String::as_str), Some("baz"));
    /// ```
    pub fn custom_args<T>(mut self, args: T) -> MessageBuilder
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.append_personalization();
        self.0.personalizations[0].custom_args.extend(args);
        self
    }

    /// Sets the "send at" timestamp for the first personalization of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().send_at(1555890183).build();
    /// assert_eq!(message.personalizations[0].send_at, Some(1555890183));
    /// ```
    pub fn send_at(mut self, timestamp: i64) -> MessageBuilder {
        self.append_personalization();
        self.0.personalizations[0].send_at = Some(timestamp);
        self
    }

    /// Sets the "from" email address for the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().from("foo@example.com").build();
    /// assert_eq!(message.from.as_ref().unwrap().email, "foo@example.com");
    /// assert_eq!(message.from.as_ref().unwrap().name, None);
    /// ```
    pub fn from<T>(mut self, email: T) -> MessageBuilder
    where
        T: Into<String>,
    {
        self.0.from = Some(EmailAddress::new(email));
        self
    }

    /// Sets the "from" email address for the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().from_with_name("foo@example.com", "Peter").build();
    /// assert_eq!(message.from.as_ref().unwrap().email, "foo@example.com");
    /// assert_eq!(message.from.as_ref().unwrap().name, Some("Peter".to_owned()));
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn from_with_name<T, U>(mut self, email: T, name: U) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.0.from = Some(EmailAddress::new_with_name(email, name));
        self
    }

    /// Sets the default global subject for all personalizations of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().global_subject("hello world").build();
    /// assert_eq!(message.subject, Some("hello world".to_owned()));
    /// ```
    pub fn global_subject<T>(mut self, subject: T) -> MessageBuilder
    where
        T: Into<String>,
    {
        self.0.subject = Some(subject.into());
        self
    }

    /// Adds a text content (with MIME type "text/plain") to the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().content("hello world").build();
    /// assert_eq!(message.contents[0].mime_type, "text/plain");
    /// assert_eq!(message.contents[0].value, "hello world");
    /// ```
    pub fn content<T>(mut self, text: T) -> MessageBuilder
    where
        T: Into<String>,
    {
        self.0.contents.push(Content {
            mime_type: "text/plain".to_owned(),
            value: text.into(),
        });

        self
    }

    /// Adds a content with the given MIME type to the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().content_with_type("hello world", "text/plain").build();
    /// assert_eq!(message.contents[0].mime_type, "text/plain");
    /// assert_eq!(message.contents[0].value, "hello world");
    /// ```
    pub fn content_with_type<T, U>(mut self, content: T, mime_type: U) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.0.contents.push(Content {
            mime_type: mime_type.into(),
            value: content.into(),
        });

        self
    }

    /// Adds the given content to the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// use azure_functions::send_grid::Content;
    /// let message = MessageBuilder::new()
    ///     .contents(
    ///         vec![
    ///             Content{ mime_type: "text/plain".to_owned(), value: "hello world".to_owned() }
    ///         ])
    ///     .build();
    /// assert_eq!(message.contents[0].mime_type, "text/plain");
    /// assert_eq!(message.contents[0].value, "hello world");
    /// ```
    pub fn contents<T>(mut self, contents: T) -> MessageBuilder
    where
        T: IntoIterator<Item = Content>,
    {
        self.0.contents.extend(contents);
        self
    }

    /// Adds an attachment to the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().attachment("hello.txt", "text/plain", "aGVsbG8gd29ybGQ=").build();
    /// assert_eq!(message.attachments[0].filename, "hello.txt");
    /// assert_eq!(message.attachments[0].mime_type, "text/plain");
    /// assert_eq!(message.attachments[0].content, "aGVsbG8gd29ybGQ=");
    /// ```
    pub fn attachment<T, U, V>(mut self, filename: T, mime_type: U, content: V) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
        V: Into<String>,
    {
        self.0.attachments.push(Attachment {
            filename: filename.into(),
            mime_type: mime_type.into(),
            content: content.into(),
            ..Default::default()
        });
        self
    }

    /// Adds an attachment to the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().inline_attachment("hello.jpg", "image/jpeg", "aGVsbG8gd29ybGQ=", "img_139db99fdb5c3704").build();
    /// assert_eq!(message.attachments[0].filename, "hello.jpg");
    /// assert_eq!(message.attachments[0].mime_type, "image/jpeg");
    /// assert_eq!(message.attachments[0].content, "aGVsbG8gd29ybGQ=");
    /// assert_eq!(message.attachments[0].disposition, Some("inline".to_owned()));
    /// assert_eq!(message.attachments[0].content_id, Some("img_139db99fdb5c3704".to_owned()));
    /// ```
    pub fn inline_attachment<T, U, V, W>(
        mut self,
        filename: T,
        mime_type: U,
        content: V,
        content_id: W,
    ) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
        V: Into<String>,
        W: Into<String>,
    {
        self.0.attachments.push(Attachment {
            filename: filename.into(),
            mime_type: mime_type.into(),
            content: content.into(),
            disposition: Some("inline".to_owned()),
            content_id: Some(content_id.into()),
        });
        self
    }

    /// Adds multiple attachments to the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// use azure_functions::send_grid::Attachment;
    /// let message = MessageBuilder::new()
    ///     .attachments(
    ///         vec![
    ///             Attachment{ filename: "hello.txt".to_owned(), mime_type: "text/plain".to_owned(), content: "aGVsbG8gd29ybGQ=".to_owned(), ..Default::default() }
    ///         ])
    ///     .build();
    /// assert_eq!(message.attachments[0].filename, "hello.txt");
    /// assert_eq!(message.attachments[0].mime_type, "text/plain");
    /// assert_eq!(message.attachments[0].content, "aGVsbG8gd29ybGQ=");
    /// ```
    pub fn attachments<T>(mut self, attachments: T) -> MessageBuilder
    where
        T: IntoIterator<Item = Attachment>,
    {
        self.0.attachments.extend(attachments);
        self
    }

    /// Sets the template id for the message.
    ///
    /// # Examples
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// use azure_functions::send_grid::Attachment;
    /// let message = MessageBuilder::new().template_id("id").build();
    /// assert_eq!(message.template_id, Some("id".to_owned()));
    /// ```
    pub fn template_id<T>(mut self, id: T) -> MessageBuilder
    where
        T: Into<String>,
    {
        self.0.template_id = Some(id.into());
        self
    }

    /// Appends a section substitution to the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().section("foo", "bar").build();
    /// assert_eq!(message.sections.get("foo").map(String::as_str), Some("bar"));
    /// ```
    pub fn section<T, U>(mut self, key: T, value: U) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.0.sections.insert(key.into(), value.into());
        self
    }

    /// Appends multiple sections to the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// # use std::collections::HashMap;
    /// let mut sections = HashMap::new();
    /// sections.insert("foo".to_owned(), "bar".to_owned());
    /// sections.insert("bar".to_owned(), "baz".to_owned());
    /// let message = MessageBuilder::new().sections(sections).build();
    /// assert_eq!(message.sections.get("foo").map(String::as_str), Some("bar"));
    /// assert_eq!(message.sections.get("bar").map(String::as_str), Some("baz"));
    /// ```
    pub fn sections<T>(mut self, sections: T) -> MessageBuilder
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.0.sections.extend(sections);
        self
    }

    /// Appends a category to the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().category("foo").build();
    /// assert_eq!(message.categories[0], "foo");
    /// ```
    pub fn category<T>(mut self, category: T) -> MessageBuilder
    where
        T: Into<String>,
    {
        self.0.categories.push(category.into());
        self
    }

    /// Appends multiple categories to the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().categories(vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()]).build();
    /// assert_eq!(message.categories[0], "foo");
    /// assert_eq!(message.categories[1], "bar");
    /// assert_eq!(message.categories[2], "baz");
    /// ```
    pub fn categories<T>(mut self, categories: T) -> MessageBuilder
    where
        T: IntoIterator<Item = String>,
    {
        self.0.categories.extend(categories);
        self
    }

    /// Appends a global header for all personalizations of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().global_header("foo", "bar").build();
    /// assert_eq!(message.headers.get("foo").map(String::as_str), Some("bar"));
    /// ```
    pub fn global_header<T, U>(mut self, key: T, value: U) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.0.headers.insert(key.into(), value.into());
        self
    }

    /// Appends multiple global headers for all personalizations of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// # use std::collections::HashMap;
    /// let mut headers = HashMap::new();
    /// headers.insert("foo".to_owned(), "bar".to_owned());
    /// headers.insert("bar".to_owned(), "baz".to_owned());
    /// let message = MessageBuilder::new().global_headers(headers).build();
    /// assert_eq!(message.headers.get("foo").map(String::as_str), Some("bar"));
    /// assert_eq!(message.headers.get("bar").map(String::as_str), Some("baz"));
    /// ```
    pub fn global_headers<T>(mut self, headers: T) -> MessageBuilder
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.0.headers.extend(headers);
        self
    }

    /// Appends a global custom argument for all personalizations of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().global_custom_arg("foo", "bar").build();
    /// assert_eq!(message.custom_args.get("foo").map(String::as_str), Some("bar"));
    /// ```
    pub fn global_custom_arg<T, U>(mut self, key: T, value: U) -> MessageBuilder
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.append_personalization();
        self.0.custom_args.insert(key.into(), value.into());
        self
    }

    /// Appends multiple global custom arguments for all personalizations of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// # use std::collections::HashMap;
    /// let mut args = HashMap::new();
    /// args.insert("foo".to_owned(), "bar".to_owned());
    /// args.insert("bar".to_owned(), "baz".to_owned());
    /// let message = MessageBuilder::new().global_custom_args(args).build();
    /// assert_eq!(message.custom_args.get("foo").map(String::as_str), Some("bar"));
    /// assert_eq!(message.custom_args.get("bar").map(String::as_str), Some("baz"));
    /// ```
    pub fn global_custom_args<T>(mut self, args: T) -> MessageBuilder
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.append_personalization();
        self.0.custom_args.extend(args);
        self
    }

    /// Sets the global "send at" timestamp for all personalizations of the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().global_send_at(1555890183).build();
    /// assert_eq!(message.send_at, Some(1555890183));
    /// ```
    pub fn global_send_at(mut self, timestamp: i64) -> MessageBuilder {
        self.append_personalization();
        self.0.send_at = Some(timestamp);
        self
    }

    /// Sets the batch id for the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_functions::send_grid::MessageBuilder;
    /// let message = MessageBuilder::new().batch_id("HkJ5yLYULb7Rj8GKSx7u025ouWVlMgAi").build();
    /// assert_eq!(message.batch_id.unwrap(), "HkJ5yLYULb7Rj8GKSx7u025ouWVlMgAi");
    /// ```
    pub fn batch_id<T>(mut self, id: T) -> MessageBuilder
    where
        T: Into<String>,
    {
        self.0.batch_id = Some(id.into());
        self
    }

    /// Consumes the builder and returns the SendGrid message.
    pub fn build(self) -> SendGridMessage {
        self.0
    }

    fn append_to(&mut self, address: EmailAddress) {
        self.append_personalization();
        self.0.personalizations[0].to.push(address);
    }

    fn append_cc(&mut self, address: EmailAddress) {
        self.append_personalization();
        self.0.personalizations[0].cc.push(address);
    }

    fn append_bcc(&mut self, address: EmailAddress) {
        self.append_personalization();
        self.0.personalizations[0].bcc.push(address);
    }

    fn append_personalization(&mut self) {
        if !self.0.personalizations.is_empty() {
            return;
        }

        self.0.personalizations.push(Personalization {
            ..Default::default()
        });
    }
}
