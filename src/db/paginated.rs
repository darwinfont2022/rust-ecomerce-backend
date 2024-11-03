use diesel::query_dsl::LoadQuery;
use diesel::{PgConnection, QueryId, QueryResult, RunQueryDsl};
use diesel::pg::Pg;
use diesel::query_builder::{AstPass, Query, QueryFragment};
use diesel::sql_types::BigInt;
use serde::Serialize;

pub trait Paginate: Sized {
    fn paginate(self, page: i64, size: i64) -> Paginated<Self>;
}
const DEFAULT_PER_PAGE: i64 = 10;
impl<T> Paginate for T {
    fn paginate(self, page: i64, size: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: if size <= 0 {DEFAULT_PER_PAGE} else { size },
            page: if page <= 0 { 1 } else {page},
            offset: if page == 0 { 0 } else { (page - 1) * size },
        }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
    offset: i64,
}

#[derive(Serialize, Debug)]
pub struct Page<T> {
    page: i64,
    limit: i64,
    total: i64,
    pages: i64,
    items: Vec<T>,
}
impl<T> Paginated<T> {
    pub fn load_page<'a, U>(self, conn: &mut PgConnection) -> QueryResult<Page<U>>
    where
    Self: LoadQuery<'a, PgConnection, (U, i64)>,
    U: std::marker::Sized,
    {
        let limit = self.per_page;
        let page = self.page;

        match self.load::<(U, i64)>(conn) {
            Ok(results) => {
                let total = results.first().map(|x| x.1).unwrap_or(0);
                let records = results.into_iter().map(|x| x.0).collect();
                let pages = (total as f64 / limit as f64).ceil() as i64;

                Ok(Page {
                    limit,
                    page,
                    items: records,
                    total,
                    pages,
                })
            },
            Err(e) => Err(e),
        }
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}