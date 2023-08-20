use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .col(ColumnDef::new(Users::Id).uuid().primary_key())
                    .col(ColumnDef::new(Users::FirstName).string_len(100))
                    .col(ColumnDef::new(Users::SecondName).string_len(100))
                    .col(
                        ColumnDef::new(Users::NickName)
                            .string_len(100)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Wishlists::Table)
                    .col(ColumnDef::new(Wishlists::Id).uuid().primary_key())
                    .col(ColumnDef::new(Wishlists::Name).string_len(100).not_null())
                    .col(ColumnDef::new(Wishlists::UserId).uuid().not_null())
                    .col(ColumnDef::new(Wishlists::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Wishlists::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Wishlists::Table)
                            .from_col(Wishlists::UserId)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Items::Table)
                    .col(ColumnDef::new(Items::Id).uuid().primary_key())
                    .col(ColumnDef::new(Items::WishlistId).uuid().not_null())
                    .col(ColumnDef::new(Items::SelectedById).uuid())
                    .col(ColumnDef::new(Items::Name).string_len(100).not_null())
                    .col(ColumnDef::new(Items::Description).string_len(300))
                    .col(ColumnDef::new(Items::Price).integer())
                    .col(ColumnDef::new(Items::IsHidden).boolean().not_null())
                    .col(ColumnDef::new(Items::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Items::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Items::Table)
                            .from_col(Items::WishlistId)
                            .to_tbl(Wishlists::Table)
                            .to_col(Wishlists::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Items::Table)
                            .from_col(Items::SelectedById)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Items::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Wishlists::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    FirstName,
    SecondName,
    NickName,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Wishlists {
    Table,
    Id,
    Name,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Items {
    Table,
    Id,
    WishlistId,
    SelectedById,
    Name,
    Description,
    Price,
    IsHidden,
    CreatedAt,
    UpdatedAt,
}
