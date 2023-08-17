import { Table, Theme } from "@radix-ui/themes";
import { ReactNode } from "react";

type Items = [ReactNode[], ...ReactNode[][]];

type StateTableProps = {
  items: Items;
};

export const StatTable = ({ items }: StateTableProps) => {
  const headers = items?.[0];
  const [_, ...rows] = items;

  return (
    <Theme
    /* style={{
        //@ts-ignore
        ["--scaling"]: 0.8,
      }} */
    >
      <Table.Root size="1">
        <Table.Header>
          <Table.Row>
            {headers.map((title) => (
              <Table.ColumnHeaderCell>{title}</Table.ColumnHeaderCell>
            ))}
          </Table.Row>
        </Table.Header>

        <Table.Body>
          {rows.map((row) => (
            <Table.Row>
              {row.map((cell) => (
                <Table.Cell>{cell}</Table.Cell>
              ))}
            </Table.Row>
          ))}
        </Table.Body>
      </Table.Root>
    </Theme>
  );
};
