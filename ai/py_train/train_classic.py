import polars as pl
import os
# import pandas as pd
import statsmodels.api as sm
import statsmodels.formula.api as smf
import csv

os.getcwd()

file_path = "data/ai/classic/weights.csv"

df_main = pl.read_csv("data/summaries/test_random/main_results.csv")
df_details = pl.read_csv("data/summaries/test_random/details.csv")

df_main = df_main.rename({"title": "game_title"})
df_details = df_details.rename({"score": "current_score"})

df = df_details.join(df_main.select(["game_title", "player_id", "score"]), on=["game_title", "player_id"], how="left")


for i in range(5):
    df = df.with_columns(pl.col(f"drawn_resource_{i}").diff().over(["game_title", "player_id"]).alias(f"change_dr_{i}"))


df = df.to_pandas()

all_coefs = []

for i in range(5):
    formula = f'score ~ change_dr_{i} * (current_score + drawn_resource_0 + drawn_resource_1+ drawn_resource_2+ drawn_resource_3+ drawn_resource_4 + budget_resource_0 + budget_resource_1 + budget_resource_2 + budget_resource_3 + budget_resource_4)'

    model = smf.ols(formula, data=df).fit()

    # print(model.summary())

    coefficients = []

    for name, coef in model.params.items():
        if name.startswith("change_dr_"):
            coefficients.append(coef)

    all_coefs.append(coefficients)

with open(file_path, mode='w', newline='') as file:
    writer = csv.writer(file)

    writer.writerows(all_coefs)

# print(all_coefs)

# all_values = []

# for i_resource in range(5):

#     coefs = all_coefs[i_resource]
#     r_values = []

#     for row in df.iterrows():
#         value = coefs[0] + coefs[1] * df["log_id"] + coefs[2] * df["current_score"] + coefs[3] * df["drawn_resource_0"]
#         r_values.append(value)

#     all_values.append(r_values)


# for series in all_values:
#     plt.plot(series)

# plt.show()
